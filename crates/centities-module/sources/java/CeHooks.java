package dev.planeta9091.centities;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.phys.AABB;
import net.minecraft.core.BlockPos;
import net.minecraft.resources.ResourceKey;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.gameevent.GameEvent;
import net.minecraft.world.phys.Vec3;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

/**
 * Bridge class defined into the kernel loader by the c-entities module.
 * Static methods here are (a) a replace_body target (getEntities),
 * (b) prefix probes (add/remove/box — the original body always runs after),
 * or (c) natives (Rust).
 *
 * FALLBACK CONTRACT: when ACTIVE is false every woven path behaves exactly
 * like vanilla (the e2e differential test proves it before arming).
 *
 * PARITY GATE (iteration 4, the owner's hard requirement: the module must
 * never be slower than vanilla). The honest A/B (doc 08) proved the fast
 * path pays a fixed ~8.4us JNI+native cost per query — it can only win on
 * queries whose vanilla scan is comparable or larger. So the module now
 * MEASURES both paths per (world, query-volume bucket) with EMAs and routes
 * each query to whichever path is measured cheaper (hysteresis + a rare
 * re-probe so EMAs cannot go stale). In steady state every query takes its
 * cheaper path, which bounds the module at vanilla parity by construction;
 * the win comes from the dense buckets where the exact Rust index beats the
 * section scan. If fast queries all but disappear, indexing fully suspends
 * (probes cost one volatile read) and a full resync re-arms it on demand.
 *
 * ZERO-RESOLVE (iteration 4): candidates are resolved through ARENA[id]
 * — entity refs written directly by the probes (2ns) — with an id-match
 * check for staleness and a LevelEntityGetter fallback. This removes the
 * ~190ns/candidate hash-path resolve that made dense queries unwinnable.
 *
 * PROBE BATCHING (iteration 2b): probes no longer cross JNI per event —
 * a setPos storm of 150k mobs/tick used to cost ~1-2us of JNI EACH. Probes
 * now append into plain Java arrays (multi-producer, atomic cursor); the
 * whole batch is applied by ONE native call at the next query (or at 3/4
 * capacity). Removes are batched the same way (op bit in the packed id).
 * Queries read a ThreadLocal long[] that the native fills in place —
 * no jlongArray allocation per query (the per-query alloc was measurable
 * GC pressure at ~100k queries/tick).
 */
public final class CeHooks {
    private CeHooks() {}

    /** Kill switch. Flipped to false by any anomaly; never re-armed in-process. */
    public static volatile boolean ACTIVE = false;

    public static volatile long queriesFast = 0, queriesMirror = 0;
    public static volatile long probesAdd = 0, probesRemove = 0, probesBox = 0;
    public static volatile long batchesFlushed = 0, batchEntries = 0;
    // Phase timings (nanos, cumulative): where the fast path spends time.
    public static volatile long fastNanos = 0, queryNanos = 0, resolveNanos = 0;
    public static volatile long candidatesSeen = 0, candidatesResolved = 0;
    // Mirror-reason counters (diagnostics): why the fast path was skipped.
    public static volatile long mNotServerLevel = 0, mBadWorld = 0, mNotReady = 0;
    public static volatile long mRegrowFail = 0, mDesync = 0, mDisabled = 0;
    public static volatile long lastError = 0;
    private static volatile String lastErrorText = "";
    // Parity-gate counters (diagnostics).
    public static volatile long arenaMiss = 0, staleCand = 0;
    public static volatile long suspends = 0, resumes = 0, resyncEnts = 0;
    public static volatile long fastProbes = 0, mirrorProbes = 0; // re-probe traffic
    public static volatile long mOffThread = 0; // fast path refused: off-thread caller

    // ------------------------------------------------------------------
    // 4b.2a ZERO-JNI ROWS — native-published per-cell id rows. The native
    // publisher (Rust) writes, for every index cell whose id list changed,
    // one long[] into ROWS (indexed by a direct cell-coord hash; collisions
    // simply make both cells' rows invalid — they lose nothing but the row
    // path). Layout: [0]=cell key, [1]=(publish stamp<<11)|idCount,
    // [2..]=ids homed in that cell at publish time. PUBLISH_SEQ changes ONLY
    // on full index reset; between resets the native side keeps the invariant
    // "stamp == PUBLISH_SEQ implies the row is exact" by republishing every
    // cell whose id list changed before the next query can read it.
    // ROW_PATH_ON is a config kill switch (-Dcentities.rowPath=false).
    // ------------------------------------------------------------------
    public static final long[][] ROWS = new long[1 << 23][]; // [wid:3][cx:10][cz:10]
    public static volatile long PUBLISH_SEQ = 0; // native stamps rows + this
    public static volatile int OVERSIZE_POP = 0; // native oversize population
    public static final boolean ROW_PATH_ON =
            !"false".equals(System.getProperty("centities.rowPath", "true"));
    private static final int ROW_CAP = 510;      // must match the native ROW_CAP
    /** L4c graded row total: mid-density rows (spread-100k: 100-300 candidates)
     *  scan in Java at ~40-70ns/candidate — ~3x cheaper than the native query's
     *  fixed JNI toll. Genuinely dense cells (mobs-25k ~700, items-100k ~830 per
     *  cell) still bail to the native Rust filter. */
    private static final int ROW_TOTAL_CAP = 512;

    // Row-path diagnostics: hits and every distinct miss reason.
    public static volatile long rowFast = 0;
    public static volatile long rowMulti = 0, rowNoRow = 0, rowKey = 0, rowStale = 0;
    public static volatile long rowPending = 0, rowBig = 0, rowNoPub = 0, rowOv = 0;
    public static volatile long rowEmpty = 0, rowHomeMiss = 0, rowStaleCand = 0;
    public static volatile long rowWinSkip = 0; // L4c: candidates skipped by the packed x-window

    // ------------------------------------------------------------------
    // L2a RESTING ITEMS — the woven ItemEntity.tick guard calls itemSkip
    // every tick. true = skip the physics (the woven body jumps straight to
    // the merge gate at bytecode 432; the vanilla tail — merge scan, age,
    // despawn event — still runs with full private access), false = run the
    // full vanilla tick.
    //
    // WAKE CONTRACT (exactness): a resting item's physics depend only on
    // blocks and its own position/velocity. Every block write funnels
    // through ServerLevel.setBlock → probeWake bumps the per-cell epoch;
    // any move of the item itself fires probeBox (the position changed — a
    // disturbance by definition); removal fires probeRemove. A resting item
    // whose cell epoch changed runs ONE full vanilla tick, then may rest
    // again. Entity pushes (boats/pistons moving the item) surface as box
    // probes; explosions that only knock back without breaking blocks are
    // covered by the ItemEntity.hurt probe (see probeHurt).
    // Residual, documented gap: Paper's Entity-level despawnTime (a private
    // final set only via API, never for vanilla items) is checked inside the
    // skipped super.tick — vanilla item despawn (age vs despawnRate, incl.
    // the Bukkit ItemDespawnEvent) runs in the vanilla tail and stays exact.
    // ------------------------------------------------------------------
    public static final boolean REST_ON =
            !"false".equals(System.getProperty("centities.rest", "true"));
    // Sized to the ARENA (1 << 22) — declared with a literal because static
    // initializers run in declaration order and ARENA is declared below.
    public static final int[] REST_SLOT = new int[1 << 22]; // packed wake slot
    public static final int[] REST_GEN = new int[1 << 22];  // epoch+1 at rest; 0 = not resting
    public static final int[] WAKE_EPOCH = new int[1 << 19];     // [wid:3][cx:8][cz:8]
    // Churn backoff (paired-CI spread): items near wandering zombies get
    // woken before their next tick FOREVER — marking them cost 19.5M useless
    // marks per run (each mark = cell math + setOldPosAndRot) while restFast
    // stayed 0. The wake probes count per-id wakes; a churny item stops
    // being marked, and each clean full tick decays the counter (the zone
    // heals when the pusher leaves).
    public static final int[] REST_CHURN = new int[1 << 22];
    public static volatile long restFast = 0, restMarks = 0, restWakes = 0;
    // Wake-source attribution (which probe cleared a resting item).
    public static volatile long wakesDelta = 0, wakesBox = 0, wakesSelf = 0;

    // SELF-TICK window (paired with the wake-on-impulse probe): the marking
    // tick's own physics (friction/landing damping at tick() bytecodes
    // 339/373) calls setDeltaMovement microseconds AFTER itemSkip set the
    // rest mark — without this window the mark was wiped by its own tick
    // every time (measured: restFast=0, 1.8M self-wakes on items-100k) and
    // NO item ever rested. An external push cannot land inside the window:
    // the server thread is inside THIS item's tick() until it returns, and
    // the self-physics calls all sit in the first ~10us of the body (the
    // merge scan that follows does not change velocity). 50us is generous.
    private static volatile int SELF_TICK_MASK = -1;
    private static volatile long SELF_TICK_UNTIL = 0;

    // Why an active item fails rest-marking (diagnostics for the items that
    // keep full-ticking forever — the rest guard's residual CPU cost).
    public static volatile long restFailGround = 0, restFailDelay = 0,
            restFailAge = 0, restFailFluid = 0, restFailVel = 0, restFailChurn = 0;

    public static volatile long restChecks = 0;
    // The L2 rest-skip itemSkip body was REMOVED by design (owner directive:
    // "мы оптимизируем, ускоряем на Rust", not "не тикать"). The name is
    // reused by the L6 native-tick collector below (the woven call site is
    // identical).

    // ------------------------------------------------------------------
    // L3 RUST MERGE BROAD-PHASE — the woven ItemEntity.mergeWithNeighbours
    // guard calls mergeScanNative on every invocation. true = the fast path
    // performed the ENTIRE neighbour merge (or proved it a vanilla no-op);
    // false = run the vanilla body unchanged (exact mirror).
    //
    // WHY: a merge scan is getEntitiesOfClass(ItemEntity, 1x0.25x1 box).
    // Vanilla's moonrise scan pays O(section) AABB tests per scan — with
    // bench items-100k piling ~25 items/block (~6500/section) that measured
    // ~95us/scan; and the resting fast path keeps many more items in the
    // mergable (count<max) state than vanilla collapses into full stacks,
    // so scan CPU hit ~32% of the tick (JFR phase-split, vs 2.8% vanilla).
    // The Rust index answers the same query with exact per-entity AABB
    // intersection over the covered cells only — a handful of candidates,
    // no section scan.
    //
    // EXACTNESS CONTRACT: candidates = every ItemEntity whose CURRENT AABB
    // intersects the merge box (Rust index, property-tested vs brute force)
    // + the vanilla predicate (other != this && isMergable, applied in Java)
    // + the vanilla loop body (isMergable re-check, fixItemsMergingThroughWalls
    // clip, tryToMerge with full private access via MethodHandle). Any
    // uncertainty (index not ready, off-thread, regrow/desync, MH failure)
    // returns false — the vanilla body runs. Documented semantic note: the
    // candidate ORDER may differ from vanilla's section order; when one scan
    // can merge with several neighbours, WHICH partner absorbs first may
    // differ, but every merge performed is vanilla-legal and per-pile totals
    // are conserved (the A/B gate tracks the observable state).
    // ------------------------------------------------------------------
    public static final boolean MERGE_ON =
            !"false".equals(System.getProperty("centities.mergeFast", "true"));
    public static volatile long mergeChecks = 0, mergeSelf = 0, mergeFast = 0;
    public static volatile long mergeMirror = 0, mergeCand = 0;

    private static volatile java.lang.invoke.MethodHandle MH_TRY_MERGE;
    private static volatile java.lang.invoke.MethodHandle MH_IS_MERGABLE;
    private static volatile boolean MH_FAILED = false;

    /** Lazy private-access handles: ItemEntity.tryToMerge(ItemEntity)V and
     *  ItemEntity.isMergable()Z. Runs once, on the server thread, from the
     *  first merge scan — ItemEntity is loaded by then (the caller IS one). */
    private static synchronized boolean initMergeMhs(Object sample) {
        if (MH_FAILED) return false;
        if (MH_TRY_MERGE != null) return true;
        try {
            Class<?> ie = sample.getClass();
            java.lang.reflect.Method tm = ie.getDeclaredMethod("tryToMerge", ie);
            tm.setAccessible(true);
            MH_TRY_MERGE = java.lang.invoke.MethodHandles.lookup().unreflect(tm)
                    .asType(java.lang.invoke.MethodType.methodType(void.class, ie, ie));
            java.lang.reflect.Method im = ie.getDeclaredMethod("isMergable");
            im.setAccessible(true);
            MH_IS_MERGABLE = java.lang.invoke.MethodHandles.lookup().unreflect(im)
                    .asType(java.lang.invoke.MethodType.methodType(boolean.class, ie));
            return true;
        } catch (Throwable t) {
            MH_FAILED = true;
            lastError = System.nanoTime();
            lastErrorText = "mergeMhs: " + t;
            return false;
        }
    }

    private static boolean isMergableFast(ItemEntity e) {
        try {
            return (boolean) MH_IS_MERGABLE.invokeExact(e);
        } catch (Throwable t) {
            return false;
        }
    }

    /** Vanilla predicate + class filter: other instanceof ItemEntity,
     *  other != self, other.isMergable().
     *
     *  CONSTRAINT: CeHooks is embedded into the JVM as the SINGLE class file
     *  CeHooks.class — named/anonymous nested classes are separate class
     *  files the runtime never defines (observed 2026-10-21:
     *  NoClassDefFoundError CeHooks$MergePred crashed the boot). So the
     *  predicate is a method-reference lambda (invokedynamic inside this
     *  same class — the BootstrapMethods path CeWarmup pre-warms), and the
     *  per-scan "self" lives in a static field confined to the server
     *  thread (fastQuery applies the predicate synchronously within the
     *  same call; off-thread callers take the mirror and never reach it). */
    private static ItemEntity MERGE_SELF; // server-thread confined

    private static boolean mergePredTest(Entity o) {
        if (!(o instanceof ItemEntity) || o == MERGE_SELF) return false;
        return isMergableFast((ItemEntity) o);
    }

    private static final Predicate<Entity> MERGE_PRED = CeHooks::mergePredTest;

    public static boolean mergeScanNative(ItemEntity e) {
        mergeChecks++;
        if (!MERGE_ON || !ACTIVE) return false;
        try {
            net.minecraft.world.level.Level w = e.level();
            if (!(w instanceof net.minecraft.server.level.ServerLevel)
                    || w.isClientSide) {
                return false;
            }
            if (MH_TRY_MERGE == null && !initMergeMhs(e)) return false;
            // Vanilla no-op: mergeWithNeighbours() returns immediately when
            // this item cannot merge. Proven without any query.
            if (e.isRemoved() || !isMergableFast(e)) {
                mergeSelf++;
                return true;
            }
            net.minecraft.server.level.ServerLevel sl =
                    (net.minecraft.server.level.ServerLevel) w;
            double r = w.spigotConfig.itemMerge;
            boolean horizOnly =
                    w.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
            AABB box = e.getBoundingBox().inflate(r, horizOnly ? 0.0 : r - 0.5, r);
            MERGE_SELF = e;
            List<Entity> cands;
            try {
                cands = fastQuery(sl, e, box, MERGE_PRED);
            } finally {
                MERGE_SELF = null;
            }
            if (cands == null) {
                mergeMirror++;
                return false; // index unsure — vanilla body
            }
            mergeFast++;
            mergeCand += cands.size();
            boolean walls = w.paperConfig().fixes.fixItemsMergingThroughWalls;
            for (int k = 0; k < cands.size(); k++) {
                ItemEntity other = (ItemEntity) cands.get(k);
                // Vanilla loop re-checks isMergable per candidate; the walls
                // fix is a per-candidate ray clip (paper default off).
                if (!isMergableFast(other)) continue;
                if (walls
                        && w.clipDirect(e.position(), other.position(),
                               net.minecraft.world.phys.shapes.CollisionContext.of(e))
                            == net.minecraft.world.phys.HitResult.Type.BLOCK) {
                    continue;
                }
                MH_TRY_MERGE.invokeExact(e, other);
                if (e.isRemoved()) return true; // absorbed — vanilla stops
            }
            return true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "mergeScan: " + t;
            t.printStackTrace();
            ACTIVE = false;
            return false;
        }
    }

    /** Prefix of ServerLevel.setBlock(BlockPos, BlockState, int[, int]). */
    public static void probeWake(net.minecraft.world.level.Level level,
            net.minecraft.core.BlockPos pos) {
        try {
            if (!ACTIVE || level == null || pos == null
                    || !(level instanceof net.minecraft.server.level.ServerLevel)
                    || level.isClientSide) {
                return;
            }
            // L6: the changed section's cached collision/fluid data is stale —
            // the flush invalidates it (next tick's gather sees the new blocks;
            // the vanilla_next routing self-heals this tick).
            if (ITEM_TICK_ON) {
                markSectionDirty(level, pos.getX(), pos.getY(), pos.getZ());
            }
            int wid = worldId(level);
            if (wid < 0) return;
            int x = pos.getX(), z = pos.getZ();
            int cx = cellFloor((double) x), cz = cellFloor((double) z);
            bumpWake(wid, cx, cz);
            // Boundary-adjacent cells: an edit within 1 block of a cell edge
            // can affect resting items whose boxes reach across it. A spurious
            // wake costs one vanilla tick; a missed one would be a desync.
            int lx = Math.floorMod(x, 16), lz = Math.floorMod(z, 16);
            if (lx <= 0) bumpWake(wid, cx - 1, cz);
            if (lx >= 15) bumpWake(wid, cx + 1, cz);
            if (lz <= 0) bumpWake(wid, cx, cz - 1);
            if (lz >= 15) bumpWake(wid, cx, cz + 1);
            if (lx <= 0 && lz <= 0) bumpWake(wid, cx - 1, cz - 1);
            if (lx <= 0 && lz >= 15) bumpWake(wid, cx - 1, cz + 1);
            if (lx >= 15 && lz <= 0) bumpWake(wid, cx + 1, cz - 1);
            if (lx >= 15 && lz >= 15) bumpWake(wid, cx + 1, cz + 1);
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeWake: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    private static void bumpWake(int wid, int cx, int cz) {
        WAKE_EPOCH[wakeSlot(wid, cx, cz)]++;
    }

    /** Prefix of ItemEntity.hurtServer: any damage (explosion knockback, kill
     *  commands, void) is a disturbance — drop the rest state before the hurt
     *  applies. Cheap by design: only items reach it. */
    public static volatile long hurtProbes = 0;
    public static void probeHurt(Entity e) {
        try {
            hurtProbes++; // TEMP L2a apply-check counter (also fires from tick)
            if (REST_ON && ACTIVE && e != null) {
                REST_GEN[e.getId() & AMASK] = 0;
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeHurt: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }
    // TEMP 4b.2a bring-up diagnostics (first rowKey miss snapshot).
    public static volatile long dbgExpKey = 0, dbgGotKey = 0, dbgExpSlot = -1, dbgMisses = 0;

    /** One-line diagnostic snapshot (all counters). */
    public static String stats() {
        return "fast=" + queriesFast + " mirror=" + queriesMirror
                + " (nslevel=" + mNotServerLevel + " badWorld=" + mBadWorld
                + " notReady=" + mNotReady + " regrow=" + mRegrowFail
                + " desync=" + mDesync + " disabled=" + mDisabled + ")"
                + " add=" + probesAdd + " rm=" + probesRemove + " box=" + probesBox
                + " flush=" + batchesFlushed + "/" + batchEntries
                + " fastMs=" + fastNanos / 1_000_000 + " qMs=" + queryNanos / 1_000_000
                + " resMs=" + resolveNanos / 1_000_000
                + " cand=" + candidatesSeen + "/" + candidatesResolved
                + " arenaMiss=" + arenaMiss + " stale=" + staleCand
                + " susp=" + suspends + " res=" + resumes + " resync=" + resyncEnts
                + " reprobe=" + fastProbes + "/" + mirrorProbes
                + " gate=" + gateStats()
                + " row=" + rowFast + "/" + rowMulti + "/" + rowStale + "/" + rowPending
                + "/" + rowNoRow + "/" + rowKey + "/" + rowBig + "/" + rowNoPub
                + "/" + rowOv + "/" + rowEmpty + "/" + rowHomeMiss + "/" + rowStaleCand
                + "/w" + rowWinSkip
                + " rest=" + restFast + "/" + restMarks + "/" + restWakes + "/" + restChecks + "/" + hurtProbes
                + " wakes=" + wakesDelta + "/" + wakesBox + "/" + wakesSelf
                + " restF=" + restFailGround + "/" + restFailDelay + "/" + restFailAge
                + "/" + restFailFluid + "/" + restFailVel + "/" + restFailChurn
                + " merge=" + mergeChecks + "/" + mergeSelf + "/" + mergeFast
                + "/" + mergeMirror + "/" + mergeCand
                + " offT=" + mOffThread
                + " dbgExp=" + Long.toHexString(dbgExpKey) + "/" + Long.toHexString(dbgGotKey)
                + "/" + dbgExpSlot + "/" + dbgMisses
                + " item=" + itemChecks + "/" + itemNative + "/" + itemResync
                + "/" + itemVanillaNext + "/" + itemAbort + "/" + itemFlushes
                + " disp=" + dispChecks + "/" + dispNative + "/" + dispInactive
                + "/" + dispPass + "/" + dispGate + "/r" + WORLD_REV
                + " indexing=" + INDEXING
                + " active=" + ACTIVE + " lastErr=" + lastErrorText;
    }

    /** L4 gate telemetry: per-bucket (every registered world) EMA/N snapshot
     *  so CI logs show WHICH buckets route where and why. Format:
     *  w<wid>b<b>:f<fastEma>us<n>/m<mirrEma>us<n>/bk<n>[;..] — buckets with
     *  zero traffic on both sides are omitted. */
    private static String gateStats() {
        StringBuilder sb = new StringBuilder(160);
        for (int wid = 0; wid < GWORLDS; wid++) {
            for (int b = 0; b < GBUCKETS; b++) {
                int gi = wid * GBUCKETS + b;
                if (FAST_N[gi] == 0 && MIRR_N[gi] == 0) continue;
                if (sb.length() > 0) sb.append(';');
                sb.append('w').append(wid).append('b').append(b)
                  .append(":f").append(FAST_EMA[gi] / 1000).append("us").append(FAST_N[gi])
                  .append("/m").append(MIRR_EMA[gi] / 1000).append("us").append(MIRR_N[gi])
                  .append("/bk").append(FAST_BACKOFF[gi]);
            }
        }
        return sb.length() == 0 ? "-" : sb.toString();
    }

    // ------------------------------------------------------------------
    // Woven (replace_body): Level.getEntities(Entity, AABB, Predicate)
    // patched-method slots: 0=this, 1=source, 2=box, 3=predicate
    // ------------------------------------------------------------------
    public static List<Entity> getEntities(
            net.minecraft.world.level.Level level,
            Entity source, AABB box, Predicate<? super Entity> predicate) {
        if (ACTIVE) {
            try {
                int gi = gateCell(level, box);
                if (gi >= 0 && wantFast(gi)) {
                    long t0 = System.nanoTime();
                    List<Entity> out = fastQuery(level, source, box, predicate);
                    long dt = System.nanoTime() - t0;
                    fastNanos += dt;
                    if (out != null) {
                        if (dt <= 0) dt = 1;
                        ema(FAST_EMA, gi, dt);
                        if (FAST_N[gi] < 1_000_000) FAST_N[gi]++;
                        // Re-probe backoff + EMA unstick (4a.1): a fast sample
                        // that lands far above the mirror price must not be
                        // paid at 1/256 forever, and an EMA trained on an
                        // empty world (bootstrap during fill) must jump to
                        // the new reality instead of creeping at 1/8.
                        long me = MIRR_EMA[gi];
                        int bk = FAST_BACKOFF[gi];
                        // L4 backoff: escalate only on a CLEAR loss (4x the
                        // mirror price — 2x was within fill-phase noise and
                        // starved probing into suspend storms); the duty
                        // cycle never drops below 1/1024. A re-probe that is
                        // CHEAPER than the EMA remembers snaps the EMA down
                        // instantly: the world got denser (the common
                        // direction), and the old 1/8 creep kept fast losing
                        // for thousands of ticks after fill.
                        if (me > 0 && dt > 4 * me) {
                            if (bk < 2) bk++;
                            if (dt > 4 * FAST_EMA[gi]) FAST_EMA[gi] = (FAST_EMA[gi] + dt) / 2;
                            // an expensive sample also breaks the latch — the
                            // world changed under it (density spike, batch)
                            FAST_LATCH[gi] = 0;
                            CHEAP_STREAK[gi] = 0;
                        } else {
                            if (me > 0 && dt < me) bk = 0; // cheap again
                            if (me > 0 && dt < FAST_EMA[gi]) FAST_EMA[gi] = dt;
                            // L4b win latch: 3 consecutive samples cheaper
                            // than the mirror EMA latch the bucket to fast
                            if (me > 0 && dt < me) {
                                if (++CHEAP_STREAK[gi] >= 3) {
                                    FAST_LATCH[gi] = 4096;
                                    CHEAP_STREAK[gi] = 0;
                                }
                            } else {
                                CHEAP_STREAK[gi] = 0;
                            }
                        }
                        FAST_BACKOFF[gi] = bk;
                        // L4d: the L4c latch release (drop the latch when the
                        // stale fast EMA stops beating the mirror EMA) is
                        // REVERTED — it was a self-fulfilling trap. CI mobs-25k:
                        // releasing the latch stopped the fast stream, the JNI
                        // path went cold (re-probes priced at ~348us vs ~13us
                        // warm), everything fell back to a full-load mirror at
                        // 8us x 18.7k/tick, and the leg regressed -36% -> +4.6%.
                        // The spread regression L4c chased was the row CEILING
                        // (rowBig refusals -> JNI toll), never the latch: in
                        // spread w2b0 hysteresis alone routed fast (15us vs
                        // 2.9ms mirror) even with the latch armed. The existing
                        // break (dt > 4 x mirror EMA) still releases a latch
                        // that genuinely loses.
                        queriesFast++;
                        return out;
                    }
                    // fast refused (notReady/regrow/desync) — mirror is the
                    // safe answer; no EMA record (this cost is anomaly, not
                    // the mirror's steady-state price). The latch dies too:
                    // routing must not stick to a refusing path.
                    FAST_LATCH[gi] = 0;
                    queriesMirror++;
                    return mirror(level, source, box, predicate);
                }
                queriesMirror++;
                return mirrorMaybeTimed(level, source, box, predicate, gi);
            } catch (Throwable t) {
                lastError = System.nanoTime();
                lastErrorText = String.valueOf(t);
                t.printStackTrace();
                ACTIVE = false; // never break the server
            }
        } else {
            mDisabled++;
        }
        queriesMirror++;
        return mirror(level, source, box, predicate);
    }

    /** Vanilla mirror — identical to the replaced body of
     *  Level.getEntities(Entity, AABB, Predicate), Paper 1.21.8
     *  (decompiled from the shipped jar; keep in sync). */
    private static List<Entity> mirror(
            net.minecraft.world.level.Level level,
            Entity source, AABB box, Predicate<? super Entity> predicate) {
        List<Entity> ret = new ArrayList<>();
        ((net.minecraft.server.level.ServerLevel) level)
                .moonrise$getEntityLookup().getEntities(source, box, ret, predicate);
        ca.spottedleaf.moonrise.common.PlatformHooks.get()
                .addToGetEntities(level, source, box, predicate, ret);
        return ret;
    }

    // ------------------------------------------------------------------
    // Prefix probes: inserted before the original body, stack-neutral.
    // Producers may run on ANY thread (chunk-system worldgen workers spawn
    // entities too) — the batch is a multi-producer append buffer.
    // ------------------------------------------------------------------

    /** Prefix of Entity.setDeltaMovement(Vec3) AND setDeltaMovement(DDD):
     *  wake-on-impulse. A resting item's tick is skipped, so any external
     *  velocity change (a push, knockback, plugin velocity) must clear the
     *  rest state or the item would never move while vanilla slides it away.
     *  Waking a non-resting entity is one array read + branch (~3ns). */
    public static void probeDelta(Entity e) {
        try {
            // L7 first: an external velocity change on a dispatch-owned item
            // clears its native mode (the applier's own writebacks run inside
            // IN_ITEM_APPLY and do not trip this).
            l7ExternalMutation(e);
            if (e != null) {
                int m = e.getId() & AMASK;
                if (REST_GEN[m] != 0) { // resting: decide whether the impulse
                    // is self-physics (this item's own marking tick) or an
                    // external push; only the latter may break the rest.
                    if (SELF_TICK_MASK == m && System.nanoTime() < SELF_TICK_UNTIL) {
                        wakesSelf++;
                    } else {
                        REST_GEN[m] = 0;
                        wakesDelta++;
                        if (REST_CHURN[m] < 8) REST_CHURN[m]++;
                    }
                }
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeDelta: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    /** Prefix of EntityLookup.addEntity(Entity,boolean,boolean): slot 1 = entity. */
    public static void probeAdd(Entity e) {
        try {
            if (ACTIVE && INDEXING && e != null && e.level() != null && !e.level().isClientSide) {
                probesAdd++;
                ARENA[e.getId() & AMASK] = e;
                batchUpsert(e.getId(), worldId(e.level()), e.getBoundingBox(), false);
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeAdd: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    /** Prefix of EntityLookup.removeEntity(Entity): slot 1 = entity. */
    public static void probeRemove(Entity e) {
        try {
            if (ACTIVE && INDEXING && e != null) {
                probesRemove++;
                ARENA[e.getId() & AMASK] = null;
                REST_GEN[e.getId() & AMASK] = 0; // gone: drop the rest state
                batchRemove(e.getId());
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeRemove: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    /** Prefix of Entity.setBoundingBox(AABB): the NEW box arrives as an arg.
     *  This is the SINGLE choke point for box changes in 1.21.8:
     *  setPos(DDD) -> setPosRaw(DDD,Z) -> makeBoundingBox -> setBoundingBox
     *  (javap-verified; skipped only for static HangingEntities — covered by
     *  probeAdd — and for no-op positions). */
    public static void probeBox(Entity e, AABB newBox) {
        try {
            // L7 first (cheapest checks inside): an external box change on a
            // dispatch-owned item clears its native mode. Runs even when the
            // query index is suspended (INDEXING false) — correctness does
            // not depend on the index.
            l7ExternalMutation(e);
            if (ACTIVE && INDEXING && e != null && newBox != null && e.level() != null && !e.level().isClientSide) {
                probesBox++;
                ARENA[e.getId() & AMASK] = e;
                if (REST_GEN[e.getId() & AMASK] != 0) {
                    REST_GEN[e.getId() & AMASK] = 0; // a box change is a disturbance
                    wakesBox++;
                }
                // Row-neutral classification (4b.2a): a box change WITHIN one
                // home cell cannot alter any row's id list — the row path's
                // live-box filter covers it — so the row-path pending scan
                // may skip the entry. Without this, a moving entity's own
                // pending probe would force its every query onto the native
                // path (observed: falling items never hit the row path).
                AABB oldBox = e.getBoundingBox();
                boolean rowNeutral = oldBox != null
                        && cellOfBox(oldBox) == cellOfBox(newBox);
                batchUpsert(e.getId(), worldId(e.level()), newBox, rowNeutral);
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "probeBox: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    // ------------------------------------------------------------------
    // Probe batch: packed entries [world:16][id:32] + 6 doubles.
    // SINGLE-PRODUCER discipline: only the "Server thread" appends (that is
    // where every tick-time move/add/remove happens); probes from other
    // threads (worldgen workers spawning entities) take the rare direct-JNI
    // path under the index mutex. This makes the batch race-free: the
    // producer and the flusher are the same thread.
    // ------------------------------------------------------------------
    private static final int BATCH_CAP = 1 << 16;
    /** L4e: drain the pending probe batch EVERY 64 entries (was: only at
     *  3/4 of the 64k cap). The old policy let the batch grow for whole
     *  ticks when no fast query drained it (mobs-25k all-mirror: ~865
     *  entries between drains), so the RARE fast query paid the entire
     *  backlog — apply + dirty-row publish at cold-JIT prices (~348us per
     *  re-probe vs ~13us warm) — which made the EMAs honestly prefer the
     *  mirror and starved the fast path into the cold trap permanently.
     *  Draining at probe time keeps queries pure (~2-8us) and keeps the
     *  pending batch within the row path's PENDING_CAP, so the zero-JNI row
     *  path stays servable between drains. The total apply+publish work is
     *  invariant — only its placement moves from queries to probes. */
    private static final int DRAIN_CAP = 64;
    private static final long[] B_IDS = new long[BATCH_CAP];
    private static final double[] B_BOX = new double[BATCH_CAP * 6];
    private static int bN = 0;
    private static final ThreadLocal<Boolean> SERVER_THREAD =
            ThreadLocal.withInitial(() -> "Server thread".equals(Thread.currentThread().getName()));

    private static void batchUpsert(int id, int wid, AABB b, boolean rowNeutral) {
        if (!SERVER_THREAD.get()) {
            nativeBoxDirect(id, wid,
                    b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
            return;
        }
        B_IDS[bN] = ((wid & 0xFFFFL) << 32) | (id & 0xFFFF_FFFFL)
                | (rowNeutral ? ROW_NEUTRAL : 0L);
        int o = bN * 6;
        B_BOX[o] = b.minX; B_BOX[o + 1] = b.minY; B_BOX[o + 2] = b.minZ;
        B_BOX[o + 3] = b.maxX; B_BOX[o + 4] = b.maxY; B_BOX[o + 5] = b.maxZ;
        bN++;
        if (bN >= DRAIN_CAP) flush();
    }

    /** Batch entry flag (bit 62): the upsert keeps the entity's home cell
     *  (box changed WITHIN one cell) — it cannot change any row's id list,
     *  so the row-path pending scan may skip it. Bits 32..47 = world,
     *  0..31 = id, 63 = removal, 62 = row-neutral. */
    private static final long ROW_NEUTRAL = 1L << 62;

    private static int cellOfBox(AABB b) {
        // packed home cell (cx & 0x3FF) | ((cz & 0x3FF) << 10), CENTER-based
        // (the index homes by center, so every home comparison must too — a
        // min-corner packing would misclassify boxes straddling a cell
        // boundary whose center is in the other cell).
        return (cellFloor((b.minX + b.maxX) * 0.5) & 0x3FF)
                | ((cellFloor((b.minZ + b.maxZ) * 0.5) & 0x3FF) << 10);
    }

    private static void batchRemove(int id) {
        if (!SERVER_THREAD.get()) {
            nativeRemoveDirect(id);
            return;
        }
        B_IDS[bN] = (1L << 63) | (id & 0xFFFF_FFFFL); // negative long = remove
        bN++;
        if (bN >= DRAIN_CAP) flush();
    }

    /** Apply the pending batch in ONE native call. Correctness: the batch is
     *  flushed before every query, so queries always see all changes made
     *  before them. Off-thread changes went through the direct path. */
    private static void flush() {
        if (bN > 0) {
            batchesFlushed++;
            batchEntries += bN;
            nativeFlush(bN, B_IDS, B_BOX);
            bN = 0;
        }
    }

    // ------------------------------------------------------------------
    // PARITY GATE (iteration L4). Owner's hard requirement: the module must
    // never be slower than vanilla. Per (world, volume-bucket) we keep an
    // EMA of measured fast-path and mirror-path nanos; every query is routed
    // to the measured-cheaper path. Fast wins only in dense buckets (the
    // exact Rust index skips the section scan's wasted intersects); small
    // queries stay vanilla. Worst case is vanilla parity minus gate
    // arithmetic (probes keep running; their cost is O(changes), not
    // O(queries)).
    //
    // L4: the old suspend/resync machinery is GONE. Measured on CI
    // (mixed-150k-mod): fast=773k vs mirror=19M with 198 suspend->resync
    // cycles re-upserting 18.9M entities. The cycle was self-sustaining:
    // the re-probe backoff starves fast (1/4096 duty), a 16k-decision window
    // then sees chosen<32 and suspends, the resync taxes ~5-15ms and
    // poisons the fast EMA, mirror serves everything, repeat. Without
    // suspend the EMA race converges in ~20 samples per bucket and the
    // worst case is a 1/256..1/1024 probe tax — no resync storms, no
    // all-mirror phases. Correctness never depended on suspend: both paths
    // are always exact, the desync guard + kill-switches remain.
    //
    // Thread-safety: gate state is heuristic — decisions happen on the
    // server thread; races from other callers degrade to a stale choice,
    // never to incorrect results (both paths are always correct).
    // ------------------------------------------------------------------
    private static final int GBUCKETS = 16; // log2 of query box volume
    private static final int GWORLDS = 8;   // worlds beyond this stay vanilla
    private static final long[] FAST_EMA = new long[GWORLDS * GBUCKETS];
    private static final long[] MIRR_EMA = new long[GWORLDS * GBUCKETS];
    private static final int[] FAST_N = new int[GWORLDS * GBUCKETS];
    private static final int[] MIRR_N = new int[GWORLDS * GBUCKETS];
    private static final int[] FAST_SEQ = new int[GWORLDS * GBUCKETS];
    private static final int[] MIRR_SEQ = new int[GWORLDS * GBUCKETS];
    /** Re-probe backoff per cell (0..2): when a re-probe lands on a much
     *  more expensive fast sample than mirror, the 1/256 probe rate itself
     *  becomes a tax (mixed-150k: 66 probes/tick × 307us = 20 ms/tick). The
     *  step doubles up to 1/1024 until fast gets cheap again; probing never
     *  stops entirely (L4: that starvation fed the suspend storms). */
    private static final int[] FAST_BACKOFF = new int[GWORLDS * GBUCKETS];
    /** L4b win latch: after 3 consecutive fast samples BEATING the mirror
     *  EMA, route this bucket to fast unconditionally for the next 4096
     *  queries. The EMA race converges too slowly in busy worlds: the fill
     *  phase poisons FAST_EMA to ~ms prices, the 1/256..1/1024 re-probe
     *  duty then needs hundreds of ticks to un-poison it, and meanwhile
     *  ~95% of queries pay the mirror (measured mixed-150k L4: fast took
     *  over around tick ~1085 of 1200 — the bucket ENDED at f26us/m335us,
     *  a 13x win nobody collected). The latch flips the routing the moment
     *  the fast path has DEMONSTRABLY won 3 samples in a row; a sample
     *  worse than 2x the mirror EMA breaks the latch immediately, so a
     *  wrong latch costs at most a bounded burst (4096 x ~40us worst case),
     *  never correctness (both paths are exact). */
    private static final int[] FAST_LATCH = new int[GWORLDS * GBUCKETS];
    private static final int[] CHEAP_STREAK = new int[GWORLDS * GBUCKETS];
    private static int gateDecisions = 0, gateFastChosen = 0;

    /** INDEXING=false would mean probes do nothing. L4: there is no suspend
     *  path anymore — the field stays as a manual kill-switch only. */
    private static volatile boolean INDEXING = true;

    private static void ema(long[] arr, int i, long sample) {
        long e = arr[i];
        arr[i] = e == 0 ? sample : e + ((sample - e) >> 3);
    }

    private static int bucketOf(AABB box) {
        double v = (box.maxX - box.minX) * (box.maxY - box.minY) * (box.maxZ - box.minZ);
        if (v < 1) return 0;
        int b = 64 - Long.numberOfLeadingZeros((long) v); // double->long saturates
        return b >= GBUCKETS ? GBUCKETS - 1 : b;
    }

    /** Returns the gate cell index (>= 0) when this query has a gate cell
     *  (fast is ALLOWED there), or -1 when it must go vanilla outright
     *  (non-server level, unknown world, suspended). The cell is needed by
     *  BOTH paths: fast records its EMA here, mirror samples its price here
     *  — that feedback is what keeps the routing on the cheaper side. */
    private static int gateCell(net.minecraft.world.level.Level level, AABB box) {
        if (!(level instanceof net.minecraft.server.level.ServerLevel)) {
            mNotServerLevel++;
            return -1;
        }
        int wid = worldId(level);
        if (wid < 0 || wid >= GWORLDS) {
            mBadWorld++;
            return -1; // no gate cell — vanilla, guaranteed parity
        }
        return wid * GBUCKETS + bucketOf(box);
    }

    /** The per-cell routing decision. FAST_N < 16 bootstraps with measured
     *  probes; afterwards a 1/256..1/1024 re-probe keeps EMAs from going
     *  stale and the comparison runs with hysteresis (fast must be < 80% of
     *  mirror). L4: no suspend side effect — probing NEVER stops, so a
     *  world that gets denser over time converges onto the fast path in
     *  ~20 samples instead of being stuck in mirror forever. */
    private static boolean wantFast(int gi) {
        boolean fast;
        if (FAST_LATCH[gi] > 0) {
            FAST_LATCH[gi]--;
            fast = true; // latched: the fast path proved itself 3x in a row
        } else if (FAST_N[gi] < 16) {
            fast = true; // bootstrap: a few measured probes per bucket
        } else {
            int step = 256 << Math.min(FAST_BACKOFF[gi], 2); // 256..1024
            if ((++FAST_SEQ[gi] & (step - 1)) == 0) {
                fast = true; // periodic re-probe (backed off): EMAs must not go stale
                fastProbes++;
            } else {
                fast = MIRR_N[gi] >= 8 && FAST_EMA[gi] * 4 < MIRR_EMA[gi] * 5; // hysteresis
            }
        }
        gateDecisions++;
        if (fast) {
            gateFastChosen++;
        }
        return fast;
    }

    /** Mirror with sampled timing: the EMA needs mirror prices to route.
     *  gi >= 0 means the query has a gate cell (per-bucket sampling
     *  applies); gi < 0 means vanilla outright (nothing to price). */
    private static List<Entity> mirrorMaybeTimed(
            net.minecraft.world.level.Level level,
            Entity source, AABB box, Predicate<? super Entity> predicate, int gi) {
        boolean sample;
        if (gi < 0) {
            sample = false; // vanilla-outright query: no bucket to price
        } else if (MIRR_N[gi] < 64) {
            sample = true;
        } else {
            sample = (++MIRR_SEQ[gi] & 255) == 0;
            if (sample) mirrorProbes++;
        }
        if (!sample) {
            return mirror(level, source, box, predicate);
        }
        long t0 = System.nanoTime();
        List<Entity> ret = mirror(level, source, box, predicate);
        long dt = System.nanoTime() - t0;
        if (dt <= 0) dt = 1;
        if (gi >= 0) {
            ema(MIRR_EMA, gi, dt);
            if (MIRR_N[gi] < 1_000_000) MIRR_N[gi]++;
        }
        return ret;
    }

    // ------------------------------------------------------------------
    // Zero-resolve candidate arena. Entity ids come from a global monotonic
    // counter, so id & AMASK is stable for the id's whole lifetime; the
    // id-match check turns any wrapped/stale slot into a clean fallback.
    // ------------------------------------------------------------------
    private static final Entity[] ARENA = new Entity[1 << 22];
    private static final int AMASK = ARENA.length - 1;

    // ------------------------------------------------------------------
    // Fast query: flush pending probes, Rust exact query -> resolve ids.
    // Returns null => caller takes the mirror path.
    // ------------------------------------------------------------------
    private static final ThreadLocal<long[]> QBUF =
            ThreadLocal.withInitial(() -> new long[1024]);

    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, Entity source,
            AABB box, Predicate<? super Entity> predicate) {
        if (!(level instanceof net.minecraft.server.level.ServerLevel sl)) {
            mNotServerLevel++;
            return null;
        }
        // The fast path touches server-thread-only state (batch arrays, gate
        // EMAs, published rows). Off-thread callers (async plugin queries)
        // take the vanilla mirror — exact parity for them, and it removes a
        // pre-existing batch-array race.
        if (!SERVER_THREAD.get()) {
            mOffThread++;
            return null;
        }
        int wid = worldId(sl);
        if (wid < 0) {
            mBadWorld++;
            return null;
        }

        // 4b.2a ZERO-JNI ROW PATH: single-cell boxes with a fresh published
        // row are served entirely in Java — no JNI crossing, no index lock,
        // no flush (the pending-batch scan below makes that exact). Any miss
        // falls through to the flush-merged native query.
        if (ROW_PATH_ON && PUBLISH_SEQ != 0) {
            if (OVERSIZE_POP != 0) {
                rowOv++; // native oversize entities: exact scan is native-only
            } else {
                List<Entity> out = rowQuery(sl, wid, source, box, predicate);
                if (out != null) return out;
            }
        } else if (PUBLISH_SEQ == 0) {
            rowNoPub++;
        }

        // FLUSH-MERGED QUERY (iteration 3): the pending probe batch is applied
        // by the SAME native call that runs the query — one JNI crossing and
        // one index-lock acquisition per query (2b paid two each: a flush
        // before every query was ~1.7M flushes for ~1.9M queries).
        // batchN>0 only on the first attempt; retries must NOT re-apply.
        // 4b.1: the INNER phase splits (query vs resolve nanos) are sampled
        // 1-in-1024 — two System.nanoTime() calls per query (~40-50ns) were
        // pure measurement tax on the path the EMA prices; the outer dt in
        // getEntities stays exact (it is the pricing signal).
        long[] ids = QBUF.get();
        int bn = bN;
        boolean prof = (queriesFast & 1023) == 0;
        long t1 = prof ? System.nanoTime() : 0L;
        int n = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                ids, bn, B_IDS, B_BOX);
        if (prof) queryNanos += System.nanoTime() - t1;
        if (n >= 0 && bn > 0) {
            bN = 0;
            batchesFlushed++;
            batchEntries += bn;
        }
        for (int tries = 0; n >= 0 && n > ids.length && tries < 3; tries++) {
            ids = new long[n];
            QBUF.set(ids);
            n = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                    ids, 0, B_IDS, B_BOX);
        }
        if (n < 0) {
            mNotReady++;
            return null; // index not ready — mirror
        }
        if (n > ids.length) {
            mRegrowFail++;
            return null; // regrow failed 3x — mirror
        }

        ArrayList<Entity> out = new ArrayList<>(Math.min(n, 64));
        int resolved = 0;
        int stale = 0;
        int srcSkip = 0;
        candidatesSeen += n;
        long t2 = prof ? System.nanoTime() : 0L;
        int srcId = source == null ? -1 : source.getId();
        for (int k = 0; k < n; k++) {
            int cid = (int) ids[k];
            if (cid == srcId) {
                // vanilla excludes exactly the source entity; a self-query
                // (item merge scans around ITSELF) can legitimately return
                // nothing but the source — that is not a desync.
                srcSkip++;
                continue;
            }
            // ZERO-RESOLVE: the probes keep refs in ARENA[id & AMASK]; the
            // id-match rejects stale/wrapped slots, the getter is only a
            // fallback (e.g. right after a resync window).
            Entity e = ARENA[cid & AMASK];
            if (e == null || e.getId() != cid) {
                arenaMiss++;
                e = sl.getEntity(cid); // LevelEntityGetter.byId — id-keyed
                if (e == null) {
                    // Not in the arena and not in the world: almost always a
                    // candidate whose remove-probe is still sitting in the
                    // batch (item merge storms) — the batch is applied at the
                    // NEXT query, so the index is momentarily ahead of the
                    // world. Vanilla never returns such an entity; skipping
                    // is the correct behavior.
                    stale++;
                    continue;
                }
            }
            resolved++;
            // NOTE: no per-candidate box.intersects — the Rust index is exact
            // (property-tested vs brute force in centities-core); the vanilla
            // predicate still applies, as it must.
            if (predicate.test(e)) {
                out.add(e);
            }
        }
        if (prof) resolveNanos += System.nanoTime() - t2;
        candidatesResolved += resolved;
        staleCand += stale;
        // Desync means the index disagrees with the WORLD about LIVING
        // entities. All-stale candidates are the normal batching window, not
        // a desync — treating them as one falsely mirrored (and could even
        // suspend indexing) during merge storms.
        if (resolved == 0 && stale == 0 && srcSkip == 0 && n > 0) {
            mDesync++;
            return null; // desync guard
        }
        return out;
    }

    // ------------------------------------------------------------------
    // 4b.2a/4b.2b zero-JNI row query. Boxes spanning up to 2x2 cells are
    // covered (4b.2b); larger fall to the native path. EXACTNESS CONTRACT vs
    // vanilla: a candidate is returned iff its CURRENT home cell is one of
    // the covered cells AND its live AABB intersects the query box AND the
    // predicate passes — exactly the set vanilla's section scan produces.
    // The rows only bound WHICH ids are examined; the home re-check +
    // live-box filter + ARENA/getter resolve make the answer exact even for
    // rows shadowed by pending (unflushed) changes:
    //   moved-out / box-shrunk rows -> home re-check drops them,
    //   removed (pending) rows     -> ARENA null + getter null -> skipped,
    //   moved-in / added (pending) -> pending-batch scan -> native path.
    // Rows partition entities by home cell (single-homing), so multi-cell
    // iteration needs no dedup. Returns null = miss -> native path.
    // ------------------------------------------------------------------
    private static int cellFloor(double x) {
        return (int) Math.floor(x) >> 4;
    }

    // Multi-cell scratch (server-thread only, reused — no per-query alloc).
    private static final long[][] MQ_ROWS = new long[4][];
    private static final int[] MQ_CELL = new int[4];
    // L4c windowed rows: the UNWRAPPED cell x-origin of each covered row
    // (world blocks) — the reference frame for the packed candidate windows.
    private static final long[] MQ_CELL_X0 = new long[4];
    // Pending row-neutral ids homed in a covered cell: their packed windows
    // are stale until the next publish, so the window test must not skip them.
    // Sized to PENDING_CAP: every pending row-neutral id of a covered cell
    // MUST be recorded, or its stale window could false-skip it.
    private static final int[] WIN_IDS = new int[64];
    private static int winN = 0;

    private static List<Entity> rowQuery(
            net.minecraft.server.level.ServerLevel sl, int wid,
            Entity source, AABB box, Predicate<? super Entity> predicate) {
        int cx0 = cellFloor(box.minX);
        int cx1 = cellFloor(box.maxX);
        int cz0 = cellFloor(box.minZ);
        int cz1 = cellFloor(box.maxZ);
        if (cx1 - cx0 > 1 || cz1 - cz0 > 1) { rowMulti++; return null; }
        int nr = 0;
        int total = 0;
        for (int cx = cx0; cx <= cx1; cx++) {
            for (int cz = cz0; cz <= cz1; cz++) {
                int packed = ((wid & 0x7) << 20) | (cx & 0x3FF) | ((cz & 0x3FF) << 10);
                long[] row = ROWS[packed];
                if (row == null) { rowNoRow++; return null; }
                long key = ((long) (wid & 0xFFFF) << 44)
                        | (((long) ((cx + (1 << 20)) & 0x3FFFFF)) << 22)
                        | (long) ((cz + (1 << 20)) & 0x3FFFFF);
                if (row[0] != key) {
                    rowKey++;
                    if (dbgMisses < 3) {
                        dbgMisses++;
                        dbgExpKey = key;
                        dbgGotKey = row[0];
                        dbgExpSlot = packed;
                    }
                    return null;
                }
                long hdr = row[1];
                if ((hdr >>> 11) != PUBLISH_SEQ) { rowStale++; return null; }
                int n = (int) (hdr & 0x7FF);
                if (n > ROW_CAP) { rowBig++; return null; } // dense-kill sentinel
                MQ_ROWS[nr] = row;
                MQ_CELL[nr] = (cx & 0x3FF) | ((cz & 0x3FF) << 10);
                MQ_CELL_X0[nr] = (long) cx << 4;
                nr++;
                total += n;
            }
        }
        // Pending (unflushed) probe batch: an upsert whose NEW home cell is
        // ANY covered cell means some row predates the entity — answering
        // from the rows would be a false negative. Everything else a pending
        // batch can do to the covered cells (move out, remove, in-cell box
        // change) is made exact by the home re-check + live-box filter below.
        if (bN > 0) {
            // L4e: bound raised 16 -> 64 to match DRAIN_CAP — probe-time
            // draining keeps bN <= 64, so the scan below stays O(64x4) while
            // the row path now serves between drains instead of refusing.
            if (bN > 64) { rowPending++; return null; } // bound the scan
            winN = 0;
            for (int i = 0; i < bN; i++) {
                long l = B_IDS[i];
                if (l < 0) continue;                 // removals resolve to null below
                if ((l & ROW_NEUTRAL) != 0) {
                    // same-cell box change: the row's ID LIST is unchanged,
                    // but the packed x-window this row was published with is
                    // now stale for that id. If its (recomputed) home is one
                    // of the covered cells, the window test must not skip it
                    // — record the id so the scan derefs it and lets the live
                    // box decide (all APPLIED changes are republished before
                    // any later query; only PENDING ones go stale).
                    int o = i * 6;
                    int home = (cellFloor((B_BOX[o] + B_BOX[o + 3]) * 0.5) & 0x3FF)
                            | ((cellFloor((B_BOX[o + 2] + B_BOX[o + 5]) * 0.5) & 0x3FF) << 10);
                    for (int r = 0; r < nr; r++) {
                        if (MQ_CELL[r] == home && winN < WIN_IDS.length) {
                            WIN_IDS[winN++] = (int) l;
                        }
                    }
                    continue;
                }
                if (((l >> 32) & 0xFFFF) != (wid & 0xFFFF)) continue;
                int o2 = i * 6;
                int home = (cellFloor((B_BOX[o2] + B_BOX[o2 + 3]) * 0.5) & 0x3FF)
                        | ((cellFloor((B_BOX[o2 + 2] + B_BOX[o2 + 5]) * 0.5) & 0x3FF) << 10);
                for (int r = 0; r < nr; r++) {
                    if (MQ_CELL[r] == home) {
                        rowPending++;
                        return null;
                    }
                }
            }
        } else {
            winN = 0;
        }
        if (total == 0) { rowEmpty++; return new ArrayList<>(0); }
        // Row-size ceiling (L4c, graded): the row path pays a per-candidate
        // home re-check + live-box test in Java (~40-70ns each), while the
        // native query pays a FIXED ~13-30us JNI+drain+publish toll and then
        // filters in Rust. The old flat 64 ceiling measured right for the
        // dense worlds it was tuned on (mobs-25k rows ~700/cell — the native
        // filter wins there), but CI spread-100k made it a regression: rows
        // of 100-300 candidates were refused into the native path at
        // ~30us/query x 6.7k/tick = ~200ms/tick, +10% p50 over vanilla.
        // 512 keeps genuinely dense cells (items-100k ~830, mobs-25k ~700)
        // on the native path while serving mid-density rows at ~10-20us.
        if (total > ROW_TOTAL_CAP) { rowBig++; return null; }
        ArrayList<Entity> out = new ArrayList<>(Math.min(total, ROW_TOTAL_CAP));
        int srcId = source == null ? -1 : source.getId();
        int stale = 0, homeMiss = 0;
        for (int r = 0; r < nr; r++) {
            long[] row = MQ_ROWS[r];
            int packed = MQ_CELL[r];
            int n = (int) (row[1] & 0x7FF);
            // L4c windowed scan: each row entry packs (id, xLo, xHi) — a
            // conservative x-interval of the entity's box in 1/1024-block
            // units relative to the ROW's cell origin (see the Rust
            // cell_scan_packed contract; floor/ceil + 2-unit slack on both
            // sides make the packed interval a strict superset of the true
            // one). The query window is quantized the same way, so a
            // candidate whose packed interval is disjoint from it provably
            // has no x overlap and cannot intersect the query box — it is
            // skipped WITHOUT dereferencing the Entity. Survivors run the
            // UNCHANGED exact path below (home re-check + live-box intersects
            // + predicate), so the window can only save work, never change
            // the answer. Typical push query: ~2.6 blocks wide in a 16-block
            // cell -> ~85% of candidates skipped at ~3ns each.
            long cellX0 = MQ_CELL_X0[r];
            int qLo = ((int) Math.floor((box.minX - cellX0) * 1024.0)) - 2 + 32768;
            int qHi = ((int) Math.ceil((box.maxX - cellX0) * 1024.0)) + 2 + 32768;
            for (int k = 0; k < n; k++) {
                long p = row[2 + k];
                int cid = (int) p;
                if (cid == srcId) continue;
                // stale-window guard: pending same-cell box changes are not
                // reflected in the packed window — deref them and let the
                // live box decide (winN is 0..2 in practice).
                boolean winTrusted = true;
                for (int w = 0; w < winN; w++) {
                    if (WIN_IDS[w] == cid) { winTrusted = false; break; }
                }
                if (winTrusted
                        && ((((int) (p >>> 48)) & 0xFFFF) < qLo
                            || (((int) (p >>> 32)) & 0xFFFF) > qHi)) {
                    rowWinSkip++;
                    continue;
                }
                Entity e = ARENA[cid & AMASK];
                if (e == null || e.getId() != cid) {
                    e = sl.getEntity(cid); // LevelEntityGetter.byId — id-keyed
                    if (e == null) { stale++; continue; }
                }
                AABB eb = e.getBoundingBox();
                // home re-check (exactness): the entity's CURRENT home must
                // still be the cell whose row we are reading; a pending
                // cross-cell move drops out here exactly like vanilla's
                // section semantics.
                if (cellOfBox(eb) != packed) {
                    homeMiss++;
                    continue;
                }
                if (eb.intersects(box) && (predicate == null || predicate.test(e))) {
                    out.add(e);
                }
            }
        }
        rowStaleCand += stale;
        rowHomeMiss += homeMiss;
        rowFast++;
        return out;
    }

    // ------------------------------------------------------------------
    // World registry. ConcurrentHashMap of Level keys keeps identity
    // semantics (Level inherits Object.equals) AND survives probes from
    // worldgen threads racing a first-time registration (the old
    // IdentityHashMap could corrupt its buckets under that race).
    // ------------------------------------------------------------------
    private static final java.util.concurrent.ConcurrentHashMap<net.minecraft.world.level.Level, Integer> WORLDS =
            new java.util.concurrent.ConcurrentHashMap<>();
    private static final java.util.concurrent.atomic.AtomicInteger NEXT_WORLD =
            new java.util.concurrent.atomic.AtomicInteger();

    // 4b.1: identity memo for worldId — every query (fast AND mirror) pays
    // gateCell → worldId, and a ConcurrentHashMap.get costs ~40-60ns while an
    // identity scan over 4 slots is ~3-5ns. Slot w is bound to THE level whose
    // world id is w, published only AFTER the WORLDS registration completed,
    // and never rebound (world ids are never reassigned) — so concurrent
    // first-writers write the same value (benign race, reference writes are
    // atomic) and a reader can never observe a torn pair: worst case it sees
    // a not-yet-published slot and takes the map path.
    private static final net.minecraft.world.level.Level[] WID_SLOTS = new net.minecraft.world.level.Level[4];

    public static int worldId(net.minecraft.world.level.Level level) {
        for (int w = 0; w < WID_SLOTS.length; w++) {
            if (WID_SLOTS[w] == level) return w;
        }
        Integer got = WORLDS.get(level);
        int w;
        if (got != null) {
            w = got;
        } else {
            int nw = NEXT_WORLD.getAndIncrement();
            Integer prev = WORLDS.putIfAbsent(level, nw);
            w = prev != null ? prev : nw;
        }
        if (w < WID_SLOTS.length && WID_SLOTS[w] == null) {
            WID_SLOTS[w] = level;
        }
        return w;
    }

    /** Kill-switch arm/disarm from the native side. */
    public static void setActive(boolean on) {
        ACTIVE = on;
        if (on && ITEM_TICK_ON) {
            // arm the L6 machinery: VarHandles + the (runtime-only) Bukkit
            // surface. A failure keeps VH_READY/BUKKIT_READY false — the
            // collectors then route EVERY item vanilla (inherent safety).
            boolean ok = initItemVh();
            ok &= initBukkit();
            if (ok) {
                cplugReady = true;
            }
        }
        if (!on) {
            cplugReady = false;
        }
    }

    /** Both init passes succeeded — L6 may arm. Volatile: read by the
     *  collectors on the server thread, written by the arm thread. */
    private static volatile boolean cplugReady = false;

    // ------------------------------------------------------------------
    // Natives (Rust). queryNative applies the caller's pending probe batch
    // (single lock acquisition) and runs the exact query into the caller's
    // buffer; returns the count (or the required capacity as a positive
    // value if the buffer is too small; -1 = index not ready). nativeFlush
    // applies a packed batch mid-tick when it hits 3/4 capacity: entries
    // with bit63 set are removals. The *Direct variants serve the rare
    // off-thread probes (worldgen workers) — they apply one change
    // immediately under the index mutex.
    // ------------------------------------------------------------------
    public static native int queryNative(
            int world, double x0, double y0, double z0, double x1, double y1, double z1,
            long[] out, int batchN, long[] batchIds, double[] batchBoxes);
    public static native void nativeFlush(int n, long[] ids, double[] boxes);
    public static native void nativeBoxDirect(
            long id, int world, double x0, double y0, double z0, double x1, double y1, double z1);
    public static native void nativeRemoveDirect(long id);
    public static native void nativeReset();

    // ==================================================================
    // L6 — NATIVE ITEM TICK CORE.
    // The owner directive is binding: the item tick is COMPUTED in Rust,
    // never skipped. The woven ItemEntity.tick / inactiveTick guards call
    // the collectors below; true = the vanilla body is suppressed because
    // the Rust core computes the SAME tick natively at the flush (one JNI
    // crossing per world-tick, at Level.tickBlockEntities — right after the
    // entity loop, exactly where vanilla's own ordering lands items).
    //
    // EXACTNESS: the core is a line-by-line port of ItemEntity.tick +
    // Entity.move/collide + moonrise performCollisions (f32 literal rules,
    // 1e-7 epsilons, (tickCount+id)%4 gate, floor-change merge cadence).
    // Anything the core cannot compute exactly (fluids, fire, complex
    // shapes, world border, portals, Paper despawn timers) routes vanilla —
    // those items still tick, in Java. The core re-checks the geometry
    // reasons every flush; block changes invalidate the section cache via
    // the probeWake dirty set below.
    // ==================================================================

    public static final boolean ITEM_TICK_ON = !"0".equals(System.getenv("CE_ITEM_TICK"));
    public static volatile long itemChecks = 0, itemNative = 0, itemInactive = 0,
            itemResync = 0, itemVanillaRouted = 0, itemMerge = 0, itemDespawn = 0,
            itemLand = 0, itemVoid = 0, itemVanillaNext = 0, itemAbort = 0, itemFlushes = 0;

    // ---- batch (server-thread confined; one world per tick segment) ----
    private static final int IB_CAP = 1 << 17;
    private static final long[] IB_IDS = new long[IB_CAP];
    private static final int[] IB_FLAGS = new int[IB_CAP];
    private static final double[] IB_D = new double[IB_CAP * 14]; // x,y,z,dx,dy,dz,fallDist,stuckX,stuckY,stuckZ,supX,supY,supZ,supPresent
    private static final int[] IB_I = new int[IB_CAP * 4];        // tickCount, age, pickupDelay, despawnRate
    private static int ibN = 0;

    // flag bits (must match itemcore.rs)
    public static final int IF_INACTIVE = 1;        // entry came from inactiveTick
    public static final int IF_NATIVE = 2;          // suppress vanilla: Rust computes
    public static final int IF_RESYNC = 4;          // vanilla body ran (or forced): ingest only
    public static final int IF_FRIC_OFF = 8;        // frictionState == FALSE -> f = 1.0
    public static final int IF_FORCE_VANILLA = 16;  // fresh-flag gate: vanilla this tick
    public static final int IF_ON_GROUND = 32;      // e.onGround() at collect time

    // ==================================================================
    // L7 — DISPATCH TAKEOVER.
    //
    // The vanilla dispatcher (ServerLevel.tickNonPassenger) runs, per
    // entity per tick: setOldPosAndRot (~5% of tick CPU at 100k items),
    // tickCount++/totalEntityAge++, TWO profiler pushes (one allocates a
    // capturing lambda per entity per tick), ActivationRange.checkIfActive
    // (entity-vs-player AABB work) and only THEN tick()/inactiveTick() +
    // postTick (handlePortal). The L6 tick guard already suppressed the
    // tick BODY for native items — every native item still paid this full
    // dispatcher floor.
    //
    // itemDispatch (woven as an early-return guard on tickNonPassenger)
    // suppresses the DISPATCHER itself for items the core owns. Correctness
    // contract, layered:
    //   1. MODE native (the flush re-evaluates the mode every tick);
    //   2. cheap field gates: removed / noPhysics / portal / passengers /
    //      empty stack route vanilla immediately;
    //   3. WORLD_REV gate: if ANY block changed since this item's last
    //      native tick, run the expensive fresh-flag gates (fluids, fire,
    //      despawnTime) so water flowing onto a resting item is exact on
    //      the very tick it happens; steady state costs nothing;
    //   4. external mutations: probeBox/probeDelta clear the item's mode
    //      synchronously (any setBoundingBox/setDeltaMovement outside the
    //      applier's writeback window);
    //   5. activation: a byte-faithful port of checkIfActive for items
    //      (incl. the (tickCount+id)%4 force-active rotation, the 200-tick
    //      grace and the 20-tick temporary wake with immunity probing)
    //      decides native-active vs native-inactive; its side effects are
    //      reproduced exactly;
    //   6. the 256-tick cadence belt (core-side): one full vanilla tick +
    //      re-ingest per 256 native ticks heals any residual drift window;
    //   7. the applier mirrors the suppressed dispatcher's tickCount++ /
    //      totalEntityAge++ so the Java entity stays dispatcher-fresh.
    // Any uncertainty returns false — the vanilla dispatcher runs and the
    // L6 tick guard collects exactly as before (graceful degradation to L6
    // if this weave is refused).
    // ==================================================================
    public static volatile long dispChecks = 0, dispNative = 0,
            dispInactive = 0, dispPass = 0, dispGate = 0;

    /** World revision: bumped by the flush whenever any section changed
     *  since the previous flush. MODE_REV[slot] stamps the revision that
     *  was current at the item's latest re-evaluation; a mismatch forces
     *  the expensive fresh-flag gates for that item on the next dispatch. */
    public static final int[] MODE_REV = new int[1 << 22];
    public static volatile int WORLD_REV = 0;
    /** The applier's writeback window: setPos/setDeltaMovement fired inside
     *  the flush are OUR writes, not external mutations — the box/delta
     *  probes must not clear native modes for them. */
    static boolean IN_ITEM_APPLY = false;

    /**
     * Woven EarlyReturnGuard on ServerLevel.tickNonPassenger(Entity):
     * true = the vanilla dispatcher body is suppressed (the Rust core
     * computes this item's tick natively at the flush).
     */
    public static boolean itemDispatch(Entity ent) {
        dispChecks++;
        if (!ACTIVE || !ITEM_TICK_ON || !cplugReady) return false;
        if (!(ent instanceof ItemEntity e)) return false;
        if (ibN >= IB_CAP) return false;
        try {
            if (e.level().isClientSide) return false;
            int slot = e.getId() & MAMASK;
            if (MODE_GEN[slot] != MODE_EPOCH || MODE_VAL[slot] == 0) {
                dispPass++;
                return false;
            }
            // cheap fresh-flag gates: always plain field reads
            if (e.isRemoved() || e.noPhysics
                    || e.portalProcess != null || e.portalCooldown > 0
                    || e.getItem().isEmpty()) {
                dispPass++;
                return false;
            }
            if (!e.getPassengers().isEmpty()) {
                dispPass++;
                return false;
            }
            // the world's block set changed since this item's last native
            // tick: the expensive gates decide (their L6 pre-tick role)
            if (MODE_REV[slot] != WORLD_REV) {
                dispGate++;
                if (e.isInWater() || e.isInLava() || fireTicks(e) != 0
                        || despawnTimeOf(e) >= 0) {
                    dispPass++;
                    return false;
                }
            }
            boolean active = itemCheckActive(e);
            appendDispatch(e, active);
            if (active) {
                dispNative++;
            } else {
                dispInactive++;
            }
            return true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "itemDispatch: " + t;
            ACTIVE = false;
            return false;
        }
    }

    /**
     * Byte-faithful port of ActivationRange.checkIfActive(Entity) for
     * ItemEntity (io.papermc.paper.entity.activation.ActivationRange,
     * javap-verified on the shipped 1.21.8-60 jar), INCLUDING its side
     * effects (isTemporarilyActive=false on every call; the 20-tick
     * immunity wakeup). The vanilla dispatcher increments tickCount BEFORE
     * calling checkIfActive; this guard runs pre-increment, so every
     * tickCount read is (field + 1). The portal branches are unreachable:
     * itemDispatch routes portalProcess/portalCooldown items vanilla.
     */
    private static boolean itemCheckActive(ItemEntity e) {
        if ((e.tickCount + 1 + e.getId()) % 4 == 0) return true;
        if (e.defaultActivationState) return true;
        if (e.tickCount + 1 < 200) return true; // isAlive(): isRemoved gated above
        long cur = net.minecraft.server.MinecraftServer.currentTick;
        boolean isActive = e.activatedTick >= cur;
        e.isTemporarilyActive = false;
        if (!isActive && (cur - e.activatedTick - 1L) % 20L == 0L) {
            int i = itemImmunities(e);
            if (i >= 0) {
                e.activatedTick = cur + i;
            } else {
                e.isTemporarilyActive = true;
            }
            isActive = true;
        }
        return isActive;
    }

    /**
     * The ItemEntity slice of ActivationRange.checkEntityImmunities
     * (byte-faithful): items are neither AbstractArrow nor LivingEntity and
     * their activationType is MISC (checkInactiveWakeup returns -1 for
     * MISC), so the port reduces to: fire -> 2, immunity stamp -> 1,
     * water+fluid-push -> 100, not-on-ground -> 100, on-ground-and-moving
     * -> 100, else -1 (temporarily active for this tick only).
     */
    private static int itemImmunities(ItemEntity e) {
        if (e.getRemainingFireTicks() > 0) return 2;
        long cur = net.minecraft.server.MinecraftServer.currentTick;
        if (e.activatedImmunityTick >= cur) return 1;
        if (e.isInWater() && e.isPushedByFluid()) return 100;
        if (!e.onGround()) return 100;
        if (e.getDeltaMovement().horizontalDistanceSqr() > 9.999999747378752E-6) {
            return 100;
        }
        return -1;
    }

    /** L7 dispatch append: the entry carries ONLY the id + flags — the Rust
     *  store owns the full state for IF_NATIVE entries (hydrated from the
     *  last commit). pickupDelay is re-read fresh every dispatch (plugin
     *  parity: a mid-flight change must win over the stored value; the
     *  flush compares and prefers the batch). */
    private static void appendDispatch(ItemEntity e, boolean active) {
        int i = ibN;
        ibN = i + 1;
        IB_IDS[i] = e.getId();
        IB_FLAGS[i] = IF_NATIVE | (active ? 0 : IF_INACTIVE);
        int o = i * 10;
        for (int j = 0; j < 14; j++) IB_D[o + j] = 0.0;
        IB_I[i * 4] = 0;
        IB_I[i * 4 + 1] = 0;
        IB_I[i * 4 + 2] = e.pickupDelay;
        IB_I[i * 4 + 3] = 0;
    }

    /** L7: an external mutation of this item (box or velocity change
     *  outside the applier's window) clears its native mode — the next
     *  dispatch routes it vanilla and the L6 tick guard re-ingests the
     *  fresh state. Called from probeBox/probeDelta BEFORE their indexing
     *  gates (it must fire even when the query index is suspended). */
    private static void l7ExternalMutation(Entity e) {
        if (ITEM_TICK_ON && !IN_ITEM_APPLY && e instanceof ItemEntity) {
            int slot = e.getId() & MAMASK;
            if (MODE_GEN[slot] == MODE_EPOCH) {
                MODE_VAL[slot] = 0;
            }
        }
    }

    private static int wakeSlot(int wid, int cx, int cz) {
        // [wid:3][cx:8][cz:8] = 2^19 slots — cells beyond ±2048 blocks per
        // axis wrap into other slots: a spurious wake (safe direction),
        // never a missed one.
        return (((wid & 0x7) << 16) | ((cx & 0xFF) << 8) | (cz & 0xFF)) & 0x7FFFF;
    }

    // ---- section invalidation (probeWake appends; the flush drains) ----
    private static final int SD_CAP = 256;
    private static final int[] SD_X = new int[SD_CAP];
    private static final int[] SD_Y = new int[SD_CAP];
    private static final int[] SD_Z = new int[SD_CAP];
    private static int sdN = 0;
    private static int sdWid = -1;

    private static void markSectionDirty(Level level, int x, int y, int z) {
        if (sdN == 0) {
            sdWid = worldId(level);
        }
        if (sdN >= SD_CAP) {
            return; // overflow: the epoch-safe resync cadence self-heals
        }
        SD_X[sdN] = x >> 4;
        SD_Y[sdN] = y >> 4;
        SD_Z[sdN] = z >> 4;
        sdN++;
    }

    /** Woven probe (existing): extend with the L6 section invalidation. */
    // (probeWake body below appends via markSectionDirty before its wake bump)
    private static final int[] MODE_VAL = new int[1 << 22];
    private static final int[] MODE_GEN = new int[1 << 22];
    private static volatile int MODE_EPOCH = 1;
    private static final int MAMASK = (1 << 22) - 1;

    // ---- VarHandles (lazy; the bridge class lives in the kernel loader
    // with Paper fully visible) ----
    private static volatile boolean VH_READY = false;
    private static RefField VH_REMAINING_FIRE, VH_DESPAWN_TIME,
            VH_DESPAWN_RATE, VH_STUCK, VH_FRIC_STATE;

    /** fallDistance is a public double field in 1.21.8 mojang-mapped. */
    private static float fallDistOf(Entity e) {
        return (float) e.fallDistance;
    }

    private static void setFallDist(Entity e, float v) {
        e.fallDistance = v;
    }

    /** Access to a private field. The VarHandle is preferred (proven to
     *  work for ItemEntity members by the L3 merge handles); for Entity
     *  members the MH unreflect refused even after setAccessible on the
     *  sandbox JVM — plain reflection honors the accessible flag and is
     *  guaranteed, so it is the fallback (and, for the two fields the
     *  collector reads per tick, reflection on a cached Field costs
     *  ~15-25ns which is acceptable). */
    private static final class RefField {
        final java.lang.reflect.Field f;
        final VarHandle vh;
        RefField(Class<?> start, String name) throws ReflectiveOperationException {
            Class<?> k = start;
            java.lang.reflect.Field found = null;
            while (k != null && found == null) {
                try {
                    found = k.getDeclaredField(name);
                } catch (NoSuchFieldException notHere) {
                    k = k.getSuperclass();
                }
            }
            if (found == null) throw new NoSuchFieldException(start.getName() + "." + name);
            found.setAccessible(true);
            f = found;
            VarHandle h = null;
            try {
                h = MethodHandles.lookup().unreflectVarHandle(found);
            } catch (Throwable mhRefused) {
                h = null; // plain reflection fallback
            }
            vh = h;
        }
        int getInt(Object o) {
            try {
                return vh != null ? (int) vh.get(o) : f.getInt(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        void setInt(Object o, int v) {
            try {
                if (vh != null) vh.set(o, v); else f.setInt(o, v);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        Object getObj(Object o) {
            try {
                return vh != null ? vh.get(o) : f.get(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        void setObj(Object o, Object v) {
            try {
                if (vh != null) vh.set(o, v); else f.set(o, v);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
    }

    private static synchronized boolean initItemVh() {
        if (VH_READY) return true;
        try {
            // fallDistance / onGround / mainSupportingBlockPos are PUBLIC in
            // 1.21.8 mojang-mapped: direct field access, zero overhead.
            VH_REMAINING_FIRE = new RefField(Entity.class, "remainingFireTicks");
            VH_DESPAWN_RATE = new RefField(ItemEntity.class, "despawnRate");
            VH_STUCK = new RefField(Entity.class, "stuckSpeedMultiplier");
            VH_FRIC_STATE = new RefField(ItemEntity.class, "frictionState");
            try {
                VH_DESPAWN_TIME = new RefField(Entity.class, "despawnTime");
            } catch (NoSuchFieldException notPaperOrRenamed) {
                VH_DESPAWN_TIME = null; // documented gap (same as the L2 guard)
            }
            VH_READY = true;
            return true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initItemVh: " + t;
            return false;
        }
    }

    private static int fireTicks(Entity e) {
        return VH_REMAINING_FIRE == null ? 0 : VH_REMAINING_FIRE.getInt(e);
    }

    /** TriState.FALSE without the adventure class on the compile classpath. */
    private static boolean frictionDisabled(Entity e) {
        try {
            Object st = VH_FRIC_STATE.getObj(e);
            return st != null && "FALSE".equals(((Enum<?>) st).name());
        } catch (Throwable t) {
            return false;
        }
    }

    private static int despawnTimeOf(Entity e) {
        return VH_DESPAWN_TIME == null ? -1 : VH_DESPAWN_TIME.getInt(e);
    }

    /**
     * Woven EarlyReturnGuard on ItemEntity.tick: true = the vanilla body is
     * suppressed (the Rust core computes this tick at the flush).
     */
    public static boolean itemSkip(ItemEntity e) {
        itemChecks++;
        if (!ACTIVE || !ITEM_TICK_ON || !cplugReady || ibN >= IB_CAP) return false;
        if (e.level().isClientSide) return false; // client-side ticks stay vanilla
        try {
            boolean nativeMode = MODE_GEN[e.getId() & MAMASK] == MODE_EPOCH
                    && MODE_VAL[e.getId() & MAMASK] != 0;
            // fresh-flag gates: anything the core cannot compute exactly runs
            // vanilla THIS tick (the flush re-evaluates the mode afterwards).
            if (e.isRemoved() || e.noPhysics || e.isInWater() || e.isInLava()
                    || fireTicks(e) != 0 || despawnTimeOf(e) >= 0
                    || e.getItem().isEmpty()) {
                appendItem(e, IF_FORCE_VANILLA);
                return false;
            }
            appendItem(e, (nativeMode ? IF_NATIVE : IF_RESYNC)
                    | (frictionDisabled(e) ? IF_FRIC_OFF : 0)
                    | (e.onGround() ? IF_ON_GROUND : 0));
            return nativeMode;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "itemSkip: " + t;
            ACTIVE = false;
            return false;
        }
    }

    /**
     * Woven EarlyReturnGuard on ItemEntity.inactiveTick: true = suppressed
     * (the core computes pickupDelay--/age++/despawn natively).
     */
    public static boolean itemInactiveCollect(ItemEntity e) {
        itemChecks++;
        // L6.1: inactive items BYPASS the pipeline. Vanilla inactiveTick is
        // ~15ns (pickupDelay--, age++, despawn check) — collecting + flushing
        // them costs more than it saves. The store keeps only stuck/sup/fr
        // (none change while inactive); age/pickupDelay/tickCount are re-read
        // fresh by the tick collector when the item reactivates, so a vanilla
        // inactivity window is bit-exact. The guard weave stays (the counter
        // proves the weave is live; the mode table is never consulted here).
        //
        // L7 EXCEPTION: dispatch-inactive items are OWNED by the core (the
        // dispatcher itself is suppressed). The 256-tick cadence belt flags
        // such an item VANILLA_NEXT at the flush, its next inactiveTick runs
        // vanilla — collect it HERE (once) so the flush re-ingests the
        // post-tick state and re-arms the mode. GEN==EPOCH && VAL==0
        // identifies exactly those (recently owned, currently vanilla); the
        // forever-vanilla population has a stale GEN and stays free.
        if (ACTIVE && ITEM_TICK_ON && cplugReady && ibN < IB_CAP
                && !e.level().isClientSide) {
            try {
                int slot = e.getId() & MAMASK;
                if (MODE_GEN[slot] == MODE_EPOCH && MODE_VAL[slot] == 0) {
                    appendItem(e, IF_INACTIVE | IF_RESYNC);
                }
            } catch (Throwable t) {
                lastError = System.nanoTime();
                lastErrorText = "itemInactiveCollect: " + t;
                ACTIVE = false;
            }
        }
        return false;
    }

    private static void appendItem(ItemEntity e, int flags) {
        int i = ibN;
        int id = e.getId();
        IB_IDS[i] = id;
        IB_FLAGS[i] = flags;
        Vec3 d = e.getDeltaMovement();
        int o = i * 10;
        IB_D[o] = e.getX();
        IB_D[o + 1] = e.getY();
        IB_D[o + 2] = e.getZ();
        IB_D[o + 3] = d.x;
        IB_D[o + 4] = d.y;
        IB_D[o + 5] = d.z;
        IB_D[o + 6] = fallDistOf(e);
        // stuck + supporting are store-owned; only resync entries get fresh
        // values (flush pass 1)
        IB_D[o + 7] = 0.0;
        IB_D[o + 8] = 0.0;
        IB_D[o + 9] = 0.0;
        IB_D[o + 10] = 0.0;
        IB_D[o + 11] = 0.0;
        IB_D[o + 12] = 0.0;
        IB_D[o + 13] = 0.0;
        int oi = i * 4;
        IB_I[oi] = e.tickCount; // dispatcher already ++ed
        IB_I[oi + 1] = e.age;
        IB_I[oi + 2] = e.pickupDelay;
        IB_I[oi + 3] = VH_DESPAWN_RATE.getInt(e);
        ibN = i + 1;
    }

    /**
     * Woven static prefix on Level.tickBlockEntities — runs right after the
     * entity loop of THIS level's tick: flush the batch (one JNI crossing),
     * apply the writebacks, run the exact vanilla event snippets.
     */
    public static void itemTickFlush(Level level) {
        int n = ibN;
        itemFlushes++;
        if (n == 0) return;
        ibN = 0;
        if (!ACTIVE || !ITEM_TICK_ON || !cplugReady || !(level instanceof ServerLevel)) {
            return;
        }
        if (!initBukkit()) {
            return; // mode bits never arm -> items stay vanilla
        }
        try {
            itemFlushes++;
            int wid = worldId(level);
            boolean isNether = level.dimension() == Level.NETHER;

            // pass 1: resync entries re-read state POST vanilla tick
            for (int i = 0; i < n; i++) {
                int f = IB_FLAGS[i];
                if ((f & (IF_RESYNC | IF_FORCE_VANILLA)) != 0) {
                    Entity ref = arenaRef(IB_IDS[i]);
                    if (!(ref instanceof ItemEntity)) continue;
                    ItemEntity e = (ItemEntity) ref;
                    int o = i * 10;
                    IB_D[o] = e.getX();
                    IB_D[o + 1] = e.getY();
                    IB_D[o + 2] = e.getZ();
                    Vec3 d = e.getDeltaMovement();
                    IB_D[o + 3] = d.x;
                    IB_D[o + 4] = d.y;
                    IB_D[o + 5] = d.z;
                    IB_D[o + 6] = fallDistOf(e);
                    Vec3 stuck = (Vec3) VH_STUCK.getObj(e);
                    IB_D[o + 7] = stuck == null ? 0.0 : stuck.x;
                    IB_D[o + 8] = stuck == null ? 0.0 : stuck.y;
                    IB_D[o + 9] = stuck == null ? 0.0 : stuck.z;
                    Object sup = e.mainSupportingBlockPos;
                    if (sup instanceof java.util.Optional<?> opt && opt.isPresent()
                            && opt.get() instanceof BlockPos bp) {
                        IB_D[o + 10] = bp.getX();
                        IB_D[o + 11] = bp.getY();
                        IB_D[o + 12] = bp.getZ();
                        IB_D[o + 13] = 1.0;
                    } else {
                        IB_D[o + 13] = 0.0;
                    }
                    int oi = i * 4;
                    IB_I[oi] = e.tickCount;
                    IB_I[oi + 1] = e.age;
                    IB_I[oi + 2] = e.pickupDelay;
                    IB_I[oi + 3] = VH_DESPAWN_RATE.getInt(e);
                    itemResync++;
                }
            }

            // env snapshot (once per flush)
            double[] envD = new double[6];
            envD[0] = isNether ? 0.0 : level.getMinY() + voidOffset(level);
            envD[1] = level.getWorldBorder().getMinX();
            envD[2] = level.getWorldBorder().getMaxX();
            envD[3] = level.getWorldBorder().getMinZ();
            envD[4] = level.getWorldBorder().getMaxZ();
            envD[5] = 2.0; // border routing margin
            int[] envI = new int[4];
            envI[0] = ca.spottedleaf.moonrise.common.util.WorldUtil.getMinSection(level);
            envI[1] = ca.spottedleaf.moonrise.common.util.WorldUtil.getMaxSection(level);
            envI[2] = (voidDamageEnabled(level) && !isNether) ? 1 : 0;
            envI[3] = isNether ? 0 : 1; // itemsNative: nether worlds route vanilla

            double[] outD = growD(n * 8); // dx,dy,dz,x,y,z,fallDist,landFall
            int[] outI = growI(n * 4);    // age, pickupDelay, bits, tickCount (L7)

            int rc = nativeItemTick(wid, n, IB_IDS, IB_FLAGS, IB_D, IB_I,
                    envD, envI, outD, outI);
            if (rc != 0) {
                return; // core not ready: everything stays vanilla this tick
            }
            // L7 world revision: the stamps written below record the revision
            // CURRENT BEFORE this drain — any bump after an item's stamp
            // forces its expensive fresh-flag gates on the next dispatch.
            int revBefore = WORLD_REV;
            // drain the section invalidations (next tick's gather sees the
            // new blocks; the vanilla_next routing self-heals this tick)
            if (sdN > 0 && sdWid >= 0) {
                nativeItemInvalidate(sdWid, sdN, SD_X, SD_Y, SD_Z);
                sdN = 0;
                sdWid = -1;
                WORLD_REV = revBefore + 1;
            }

            // pass 2: apply, in batch order (== vanilla entity tick order).
            // The writebacks fire setPos/setDeltaMovement → probeBox/probeDelta
            // — IN_ITEM_APPLY marks them as OUR writes (no mode clearing).
            IN_ITEM_APPLY = true;
            try {
            int epoch = MODE_EPOCH + 1;
            for (int i = 0; i < n; i++) {
                int idSlot = (int) (IB_IDS[i] & 0xFFFF_FFFF);
                int f = IB_FLAGS[i];
                int bits = outI[i * 4 + 2];
                int mSlot = (int) (IB_IDS[i] & MAMASK);
                MODE_GEN[mSlot] = epoch;
                MODE_REV[mSlot] = revBefore;
                if ((bits & (1 << 11)) != 0) { // VANILLA_NEXT
                    MODE_VAL[mSlot] = 0;
                    itemVanillaNext++;
                } else if ((bits & (1 << 12)) != 0) { // NATIVE_OK
                    MODE_VAL[mSlot] = 1;
                } else {
                    MODE_VAL[mSlot] = 0; // resync/vanilla-ran re-enters via
                                         // want_native next flush
                }
                if ((f & IF_NATIVE) == 0 || (bits & (1 << 13)) != 0) {
                    // aborted (bit 13): the core did not compute — nothing to
                    // apply, the item froze this tick and runs vanilla next.
                    if ((bits & (1 << 13)) != 0) itemAbort++;
                    continue;
                }
                Entity ref = arenaRef(IB_IDS[i]);
                if (!(ref instanceof ItemEntity)) continue;
                ItemEntity e = (ItemEntity) ref;
                // L7: mirror the suppressed dispatcher's tickCount++ and
                // totalEntityAge++ (the vanilla tickNonPassenger body was
                // skipped for dispatch-owned entries).
                e.tickCount = outI[i * 4 + 3];
                e.totalEntityAge = e.totalEntityAge + 1;
                if ((f & IF_INACTIVE) != 0) {
                    // inactiveTick semantics: pickupDelay/age/despawn only
                    e.age = outI[i * 4];
                    e.pickupDelay = outI[i * 4 + 1];
                    if ((bits & (1 << 7)) != 0) {
                        int rate = VH_DESPAWN_RATE.getInt(e);
                        // IF_NATIVE entries carry no batch despawnRate — the
                        // field IS the truth (ctor-fixed in vanilla); the
                        // age check below still gates the event.
                        boolean rateOk = (f & IF_NATIVE) != 0
                                || rate == IB_I[i * 4 + 3];
                        if (rateOk && e.age >= rate) {
                            itemDespawn++;
                            Object ev = MH_DESPAWN_EV.invoke(e);
                            Boolean cancelled = (Boolean) ev.getClass()
                                    .getMethod("isCancelled").invoke(ev);
                            if (cancelled) {
                                e.age = 0;
                            } else {
                                MH_DISCARD.invoke(e, CAUSE_DESPAWN);
                            }
                        }
                    }
                    continue;
                }
                int o = i * 8;
                e.setDeltaMovement(new Vec3(outD[o], outD[o + 1], outD[o + 2]));
                e.age = outI[i * 4];
                e.pickupDelay = outI[i * 4 + 1];
                setFallDist(e, (float) outD[o + 6]);
                boolean wantGround = (bits & (1 << 1)) != 0;
                if (wantGround != e.onGround()) {
                    e.onGround = wantGround;
                }
                if ((bits & 1) != 0) { // MOVED
                    e.setPos(outD[o + 3], outD[o + 4], outD[o + 5]);
                }
                if ((bits & (1 << 2)) != 0) { // HAS_IMPULSE (OR-only)
                    e.hasImpulse = true;
                }
                if ((bits & (1 << 5)) != 0) { // MERGE cadence -> exact L3 path
                    itemMerge++;
                    mergeScanNative(e);
                    if (e.isRemoved()) continue;
                }
                if ((bits & (1 << 7)) != 0) { // DESPAWN: exact vanilla snippet
                    int rate = VH_DESPAWN_RATE.getInt(e);
                    boolean rateOk = (f & IF_NATIVE) != 0
                            || rate == IB_I[i * 4 + 3];
                    if (rateOk && e.age >= rate) {
                        itemDespawn++;
                        Object ev = MH_DESPAWN_EV.invoke(e);
                        Boolean cancelled = (Boolean) ev.getClass()
                                .getMethod("isCancelled").invoke(ev);
                        if (cancelled) {
                            e.age = 0;
                        } else {
                            MH_DISCARD.invoke(e, CAUSE_DESPAWN);
                        }
                    }
                }
                if ((bits & (1 << 8)) != 0) { // VOID: vanilla discard
                    itemVoid++;
                    MH_DISCARD.invoke(e, CAUSE_VOID);
                }
                if ((bits & (1 << 9)) != 0) { // LAND: fallOn + gameEvent
                    itemLand++;
                    BlockPos p = e.getOnPosLegacy();
                    BlockState s = e.level().getBlockState(p);
                    s.getBlock().fallOn(e.level(), s, p, e, (float) outD[o + 7]);
                    e.level().gameEvent(GameEvent.HIT_GROUND, e.position(),
                            GameEvent.Context.of(e, s));
                }
            }
            MODE_EPOCH = epoch;
            } finally {
                IN_ITEM_APPLY = false;
            }
        } catch (Throwable t) {
            ibN = 0;
            IN_ITEM_APPLY = false;
            lastError = System.nanoTime();
            lastErrorText = "itemTickFlush: " + t;
            t.printStackTrace();
            ACTIVE = false;
        }
    }

    /** ARENA resolve with the id-match staleness check (same as queries). */
    private static Entity arenaRef(long id) {
        Entity cand = ARENA[(int) (id & 0xFFFF_FFFFL) & AMASK];
        return (cand != null && cand.getId() == id) ? cand : null;
    }

    // ---- Bukkit surface via reflection (the compile classpath carries the
    // mojang-mapped SERVER only; the API jars exist at runtime) ----
    private static volatile boolean BUKKIT_READY = false;
    private static volatile MethodHandle MH_VOID_ENABLED, MH_VOID_OFFSET, MH_DESPAWN_EV, MH_DISCARD;
    private static Object CAUSE_DESPAWN, CAUSE_VOID;

    private static synchronized boolean initBukkit() {
        if (BUKKIT_READY) return true;
        try {
            Class<?> world = Class.forName("org.bukkit.World");
            MH_VOID_ENABLED = MethodHandles.lookup().unreflect(
                    world.getMethod("isVoidDamageEnabled"));
            MH_VOID_OFFSET = MethodHandles.lookup().unreflect(
                    world.getMethod("getVoidDamageMinBuildHeightOffset"));
            Class<?> cef = Class.forName("org.bukkit.craftbukkit.event.CraftEventFactory");
            for (java.lang.reflect.Method m : cef.getDeclaredMethods()) {
                if (m.getName().equals("callItemDespawnEvent")
                        && m.getParameterCount() == 1
                        && m.getParameterTypes()[0] == ItemEntity.class) {
                    MH_DESPAWN_EV = MethodHandles.lookup().unreflect(m);
                }
            }
            Class<?> cause = Class.forName("org.bukkit.event.entity.EntityRemoveEvent$Cause");
            CAUSE_DESPAWN = cause.getField("DESPAWN").get(null);
            CAUSE_VOID = cause.getField("OUT_OF_WORLD").get(null);
            for (java.lang.reflect.Method m : Entity.class.getMethods()) {
                if (m.getName().equals("discard") && m.getParameterCount() == 1
                        && m.getParameterTypes()[0] == cause) {
                    MH_DISCARD = MethodHandles.lookup().unreflect(m);
                }
            }
            BUKKIT_READY = MH_DESPAWN_EV != null && MH_DISCARD != null
                    && CAUSE_DESPAWN != null && CAUSE_VOID != null;
            return BUKKIT_READY;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initBukkit: " + t;
            return false;
        }
    }

    private static boolean voidDamageEnabled(Level level) {
        try {
            Object w = level.getWorld();
            Object r = MH_VOID_ENABLED.invoke(w);
            return r instanceof Boolean b ? b : true;
        } catch (Throwable unavailable) {
            return true; // vanilla default: void discards below-world entities
        }
    }

    /** World.getVoidDamageMinBuildHeightOffset via reflection (Paper API). */
    private static double voidOffset(Level level) {
        try {
            Object w = level.getWorld();
            Object r = MH_VOID_OFFSET.invoke(w);
            return r instanceof Number num ? num.doubleValue() : 0.0;
        } catch (Throwable unavailable) {
            return 0.0;
        }
    }

    /**
     * Section provider (JNI up-call from the Rust cache): packs the
     * palette-driven section data. outD: 8 doubles per palette entry
     * [flag, x0,y0,z0,x1,y1,z1, friction] + [speed, water, fluid, cobweb,
     * insideSpecial, stepSpecial, fallSpecial, complex] packed as doubles;
     * outBlocks: 4096 palette indices (0xFFFF = air slot 0). Returns false
     * when the chunk is absent (load=false semantics: gather treats absent
     * as empty, exactly like vanilla).
     */
    public static boolean itemSectionData(
            int wid, int cx, int cy, int cz, boolean load,
            double[] outD, int[] outBlocks) {
        Level level = wid < WID_SLOTS.length ? WID_SLOTS[wid] : null;
        if (!(level instanceof ServerLevel sl)) return false;
        net.minecraft.world.level.chunk.LevelChunk chunk;
        try {
            if (load) {
                chunk = sl.getChunk(cx, cz);
            } else {
                chunk = sl.getChunkSource().getChunkNow(cx, cz);
                if (chunk == null) return false;
            }
        } catch (Throwable unavailable) {
            return false;
        }
        int secIdx = cy - ca.spottedleaf.moonrise.common.util.WorldUtil.getMinSection(sl);
        net.minecraft.world.level.chunk.LevelChunkSection[] sections = chunk.getSections();
        if (secIdx < 0 || secIdx >= sections.length) return false;
        net.minecraft.world.level.chunk.LevelChunkSection section = sections[secIdx];
        java.util.HashMap<BlockState, Integer> paletteIdx = new java.util.HashMap<>(32);
        int entryN = 0; // slot 0 = air
        // AIR entry — the offsets MUST match writeEntry/Rust parse_section:
        // 0=collider 1-6=box 7=friction 8=speed 9=water 10=fluid 11=cobweb
        // 12=inside 13=step 14=fall 15=shape 16=complex 17=reserved.
        // BUG (found via the 100k-fill RSS explosion, ~457k cached sections):
        // friction/speed were written at 8/9 and water was left at 1.0 — the
        // parser read air as {friction 0.0, speed 0.6, IS_WATER=1.0}, so every
        // section reported any_fluid and the core's fluid path ran on garbage.
        outD[0] = 0.0; // air entry: no collider
        outD[7] = 0.6; // friction
        outD[8] = 1.0; // speed factor
        outD[9] = 0.0; // water: NO
        outD[10] = 0.0; outD[11] = 0.0; outD[12] = 0.0;
        outD[13] = 0.0; outD[14] = 0.0; outD[15] = 0.0; outD[16] = 0.0;
        outD[17] = 0.0;
        entryN = 1;
        try {
            net.minecraft.world.level.block.state.BlockState[] states =
                    new net.minecraft.world.level.block.state.BlockState[4096];
            int nn = 0;
            for (int y = 0; y < 16; y++) {
                for (int z = 0; z < 16; z++) {
                    for (int x = 0; x < 16; x++) {
                        int local = x | (z << 4) | (y << 8);
                        BlockState st = section.states.get(x, y, z);
                        states[nn++] = st;
                        Integer pi = paletteIdx.get(st);
                        if (pi == null) {
                            pi = entryN;
                            paletteIdx.put(st, pi);
                            writeEntry(outD, pi, st);
                            entryN++;
                        }
                        outBlocks[local] = pi;
                    }
                }
            }
        } catch (Throwable t) {
            return false;
        }
        return true;
    }

    /** One palette entry -> 18 packed doubles at slot * 18. */
    private static void writeEntry(double[] outD, int slot, BlockState st) {
        int o = slot * 18;
        var shape = st.getCollisionShape(
                net.minecraft.world.level.EmptyBlockGetter.INSTANCE,
                BlockPos.ZERO, net.minecraft.world.phys.shapes.CollisionContext.empty());
        var boxes = shape.toAabbs();
        int complex = boxes.size() > 1 ? 1 : 0;
        if (boxes.size() == 1) {
            var b = boxes.get(0);
            outD[o] = 1.0;
            outD[o + 1] = b.minX; outD[o + 2] = b.minY; outD[o + 3] = b.minZ;
            outD[o + 4] = b.maxX; outD[o + 5] = b.maxY; outD[o + 6] = b.maxZ;
        } else {
            outD[o] = 0.0;
        }
        outD[o + 7] = st.getBlock().getFriction();
        outD[o + 8] = st.getBlock().getSpeedFactor();
        outD[o + 9] = (st.is(net.minecraft.world.level.block.Blocks.WATER)
                || st.is(net.minecraft.world.level.block.Blocks.BUBBLE_COLUMN)) ? 1.0 : 0.0;
        outD[o + 10] = st.getFluidState().isEmpty() ? 0.0 : 1.0;
        outD[o + 11] = st.is(net.minecraft.world.level.block.Blocks.COBWEB) ? 1.0 : 0.0;
        outD[o + 12] = overrides(st, "entityInside") ? 1.0 : 0.0;
        outD[o + 13] = overrides(st, "stepOn") ? 1.0 : 0.0;
        outD[o + 14] = overrides(st, "updateEntityMovementAfterFallOn") ? 1.0 : 0.0;
        outD[o + 15] = overridesShape(st) ? 1.0 : 0.0;
        outD[o + 16] = complex;
        outD[o + 17] = 0.0;
    }

    /** true when the block class overrides the (possibly renamed) method. */
    private static boolean overrides(BlockState st, String name) {
        try {
            Class<?> k = st.getBlock().getClass();
            while (k != null && k != net.minecraft.world.level.block.state.BlockBehaviour.class) {
                for (java.lang.reflect.Method m : k.getDeclaredMethods()) {
                    if (m.getName().equals(name)) return true;
                }
                k = k.getSuperclass();
            }
            return false;
        } catch (Throwable t) {
            return true; // uncertain -> route vanilla
        }
    }

    /** true when the block class overrides the context-sensitive shape hook. */
    private static boolean overridesShape(BlockState st) {
        try {
            Class<?> k = st.getBlock().getClass();
            while (k != null && k != net.minecraft.world.level.block.Block.class) {
                for (java.lang.reflect.Method m : k.getDeclaredMethods()) {
                    if (m.getName().equals("getCollisionShape")
                            || m.getName().equals("getShape")) return true;
                }
                k = k.getSuperclass();
            }
            return false;
        } catch (Throwable t) {
            return true;
        }
    }

    // reusable writeback buffers (server-thread confined; grown on demand)
    private static double[] OUT_D = new double[1024 * 8];
    private static int[] OUT_I = new int[1024 * 3];

    private static double[] growD(int n) {
        if (OUT_D.length < n) OUT_D = new double[Math.max(n, OUT_D.length * 2)];
        return OUT_D;
    }

    private static int[] growI(int n) {
        if (OUT_I.length < n) OUT_I = new int[Math.max(n, OUT_I.length * 2)];
        return OUT_I;
    }

    // natives (registered in lib.rs)
    public static native int nativeItemTick(
            int wid, int n, long[] ids, int[] flags,
            double[] inD, int[] inI, double[] envD, int[] envI,
            double[] outD, int[] outI);
    public static native void nativeItemInvalidate(
            int wid, int n, int[] xs, int[] ys, int[] zs);
}
