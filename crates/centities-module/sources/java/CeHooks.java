package dev.planeta9091.centities;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.phys.AABB;
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

    // Row-path diagnostics: hits and every distinct miss reason.
    public static volatile long rowFast = 0;
    public static volatile long rowMulti = 0, rowNoRow = 0, rowKey = 0, rowStale = 0;
    public static volatile long rowPending = 0, rowBig = 0, rowNoPub = 0, rowOv = 0;
    public static volatile long rowEmpty = 0, rowHomeMiss = 0, rowStaleCand = 0;

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
    public static boolean itemSkip(net.minecraft.world.entity.item.ItemEntity e) {
        restChecks++; // before the flag checks: proves the woven call executes
        SELF_TICK_MASK = e.getId() & AMASK; // open the self-physics window
        SELF_TICK_UNTIL = System.nanoTime() + 50_000;
        if (!REST_ON || !ACTIVE) return false;
        try {
            int mask = e.getId() & AMASK;
            int g = REST_GEN[mask];
            if (g != 0) {
                if (WAKE_EPOCH[REST_SLOT[mask]] + 1 == g) {
                    // vanilla Entity.tick starts with ++tickCount; the guard
                    // jump lands PAST super.tick(), so without this the
                    // counter freezes while resting — and a frozen
                    // tickCount stuck at tickCount % 40 == 0 turns the merge
                    // gate's (tickCount % 40) into "scan EVERY tick" for
                    // ~1/40 of all items (measured 12.6x merge-scan CPU on
                    // items-100k). Incrementing here keeps the merge cadence
                    // bit-identical to a vanilla resting item.
                    e.tickCount++;
                    // The vanilla prologue also decays the pickup delay
                    // (pickupDelay > 0 && != 32767 → --) BEFORE super.tick;
                    // the guard jumps past it, so mirror the decay here —
                    // otherwise a resting item's delay would freeze and it
                    // would stay un-pickable forever. 32767 (infinite) never
                    // decays in vanilla and never decays here.
                    if (e.pickupDelay > 0 && e.pickupDelay != 32767) {
                        e.pickupDelay--;
                    }
                    restFast++;
                    return true;
                }
                REST_GEN[mask] = 0; // woken: full vanilla tick this time
                restWakes++;
                return false;
            }
            // Rest candidate? Checked while the item still ticks fully.
            // The delta threshold: a landing item's residual velocity decays
            // geometrically (0.98 * damping); 1e-4 keeps a wind-down item in
            // full-tick mode for a few extra ticks — conservative direction.
            //
            // pickupDelay is NOT a physics property: a grounded item with a
            // pending pickup delay is exactly as static as one without, and
            // the skip path mirrors the vanilla prologue's decay (see above).
            // This matters enormously on real maps: unstackable drops keep
            // pickupDelay == 32767 forever, and the old `pickupDelay <= 0`
            // gate left ~50% of all items full-ticking forever (measured:
            // restF=delay ~17k/tick on items-100k, the single largest
            // residual cost).
            // Diagnostics: items that NEVER settle are the reason the rest
            // guard leaves full-tick CPU on the table; count WHICH condition
            // blocks them (same short-circuit semantics as the old single
            // predicate, order of pure reads is irrelevant).
            net.minecraft.world.phys.Vec3 d = e.getDeltaMovement();
            if (!e.onGround()) {
                restFailGround++;
                return false;
            }
            if (e.age < 0 || e.age >= 5800) {
                restFailAge++;
                return false;
            }
            if (e.isInWater() || e.isInLava()) {
                restFailFluid++;
                return false;
            }
            if (Math.abs(d.x) >= 1.0e-4 || Math.abs(d.y) >= 1.0e-4
                    || Math.abs(d.z) >= 1.0e-4) {
                restFailVel++;
                return false;
            }
            // Churn backoff: a recently-woken item may be woken again before
            // the next tick (pusher nearby) — the mark would be wasted. Each
            // full tick that reached HERE decayed the counter first, so the
            // zone heals ~4 ticks after the pusher leaves.
            if (REST_CHURN[mask] > 0) {
                REST_CHURN[mask]--;
                if (REST_CHURN[mask] >= 4) {
                    restFailChurn++;
                    return false;
                }
            }
            {
                int slot = wakeSlotOf(e);
                REST_SLOT[mask] = slot;
                REST_GEN[mask] = WAKE_EPOCH[slot] + 1;
                restMarks++;
                // CRITICAL (local A/B regression, items-100k): the skipped
                // physics also skips setOldPos — xo/yo/zo freeze at the last
                // active tick, so the merge gate's "position changed?"
                // comparison (floor(old) != floor(new)) keeps returning TRUE
                // and every resting item merge-scans every 2 ticks instead
                // of every 40 (12.6x merge-scan CPU, +19.7% p50 on 2 cores).
                // Syncing old-pos ONCE at the rest transition restores the
                // vanilla merge cadence; the item never moves while resting,
                // so one sync is exact for the whole rest window.
                e.setOldPosAndRot();
            }
            return false;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "itemSkip: " + t;
            t.printStackTrace();
            ACTIVE = false;
            return false;
        }
    }

    private static int wakeSlotOf(net.minecraft.world.entity.item.ItemEntity e) {
        AABB b = e.getBoundingBox();
        int wid = worldId(e.level());
        int cx = cellFloor((b.minX + b.maxX) * 0.5);
        int cz = cellFloor((b.minZ + b.maxZ) * 0.5);
        return wakeSlot(wid, cx, cz);
    }

    private static int wakeSlot(int wid, int cx, int cz) {
        // [wid:3][cx:8][cz:8] = 2^19 slots — cells beyond ±2048 blocks per
        // axis wrap into other slots: a spurious wake (safe direction),
        // never a missed one.
        return (((wid & 0x7) << 16) | ((cx & 0xFF) << 8) | (cz & 0xFF)) & 0x7FFFF;
    }

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
            if (!REST_ON || !ACTIVE || level == null || pos == null
                    || !(level instanceof net.minecraft.server.level.ServerLevel)
                    || level.isClientSide) {
                return;
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
                + " rest=" + restFast + "/" + restMarks + "/" + restWakes + "/" + restChecks + "/" + hurtProbes
                + " wakes=" + wakesDelta + "/" + wakesBox + "/" + wakesSelf
                + " restF=" + restFailGround + "/" + restFailDelay + "/" + restFailAge
                + "/" + restFailFluid + "/" + restFailVel + "/" + restFailChurn
                + " merge=" + mergeChecks + "/" + mergeSelf + "/" + mergeFast
                + "/" + mergeMirror + "/" + mergeCand
                + " offT=" + mOffThread
                + " dbgExp=" + Long.toHexString(dbgExpKey) + "/" + Long.toHexString(dbgGotKey)
                + "/" + dbgExpSlot + "/" + dbgMisses
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
                        } else {
                            if (me > 0 && dt < me) bk = 0; // cheap again
                            if (me > 0 && dt < FAST_EMA[gi]) FAST_EMA[gi] = dt;
                        }
                        FAST_BACKOFF[gi] = bk;
                        queriesFast++;
                        return out;
                    }
                    // fast refused (notReady/regrow/desync) — mirror is the
                    // safe answer; no EMA record (this cost is anomaly, not
                    // the mirror's steady-state price).
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
        if (bN >= BATCH_CAP - BATCH_CAP / 4) flush();
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
        if (bN >= BATCH_CAP - BATCH_CAP / 4) flush();
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
        if (FAST_N[gi] < 16) {
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
            if (bN > 16) { rowPending++; return null; } // bound the scan
            for (int i = 0; i < bN; i++) {
                long l = B_IDS[i];
                if (l < 0) continue;                 // removals resolve to null below
                if ((l & ROW_NEUTRAL) != 0) continue; // same-cell box change: row id list unchanged
                if (((l >> 32) & 0xFFFF) != (wid & 0xFFFF)) continue;
                int o = i * 6;
                int home = (cellFloor((B_BOX[o] + B_BOX[o + 3]) * 0.5) & 0x3FF)
                        | ((cellFloor((B_BOX[o + 2] + B_BOX[o + 5]) * 0.5) & 0x3FF) << 10);
                for (int r = 0; r < nr; r++) {
                    if (MQ_CELL[r] == home) {
                        rowPending++;
                        return null;
                    }
                }
            }
        }
        if (total == 0) { rowEmpty++; return new ArrayList<>(0); }
        // Row-size ceiling: the row path pays a per-candidate home re-check
        // + live-box test in Java (~30ns each), while the native query does
        // the AABB filter in Rust and returns only the exact candidates.
        // Past ~64 candidates the native path wins (measured: merge scans in
        // ~200-id rows spent ~10% of the tick in cellOfBox/rowQuery loops).
        // Rows stay the zero-JNI win for small cells.
        if (total > 64) { rowBig++; return null; }
        ArrayList<Entity> out = new ArrayList<>(Math.min(total, 64));
        int srcId = source == null ? -1 : source.getId();
        int stale = 0, homeMiss = 0;
        for (int r = 0; r < nr; r++) {
            long[] row = MQ_ROWS[r];
            int packed = MQ_CELL[r];
            int n = (int) (row[1] & 0x7FF);
            for (int k = 0; k < n; k++) {
                int cid = (int) row[2 + k];
                if (cid == srcId) continue;
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
    }

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
}
