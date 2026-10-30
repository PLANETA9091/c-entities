package dev.planeta9091.centities;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.phys.AABB;
import net.minecraft.core.BlockPos;
import net.minecraft.resources.ResourceKey;
import net.minecraft.server.level.ServerEntity;
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
    /** L37d: candidates dropped by the exact AABB re-check (the index's
     *  margin superset) — zero means the index was already exact. */
    public static volatile long marginExtra = 0;
    // Mirror-reason counters (diagnostics): why the fast path was skipped.
    public static volatile long mNotServerLevel = 0, mBadWorld = 0, mNotReady = 0;
    public static volatile long mRegrowFail = 0, mDesync = 0, mDisabled = 0;
    public static volatile long mWorldSlotClash = 0; // L15: worldId slot collision (map-path fallback)
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
    // L37c ORDER CHECK: run the vanilla mirror next to the fast path and
    // compare the result lists ELEMENT-BY-ELEMENT — logs ORDER-MISMATCH
    // (ids, sections, seqs of the first differing position) and a running
    // counter into the hooks@ line. Diagnostic only; costs a full vanilla
    // scan per checked query.
    public static final boolean ORDER_CHECK =
            "true".equals(System.getProperty("centities.orderCheck", "false"));
    public static long orderChecks = 0, orderMismatches = 0;

    public static final boolean ROW_PATH_ON =
            !"false".equals(System.getProperty("centities.rowPath", "true"));
    private static final int ROW_CAP = 510;      // must match the native ROW_CAP
    /** L4c graded row total: mid-density rows (spread-100k: 100-300 candidates)
     *  scan in Java at ~40-70ns/candidate — ~3x cheaper than the native query's
     *  fixed JNI toll. Genuinely dense cells (mobs-25k ~700, items-100k ~830 per
     *  cell) still bail to the native Rust filter. */
    private static final int ROW_TOTAL_CAP = 512;
    /** L35 narrow-shape total cap: a query box whose X and Z spans are both
     *  <= 8 blocks (mob collision ~0.6-1.9, item merge scans ~1-2.5) pays a
     *  packed-x-window scan of ~3-4ns/entry that skips 75-96% of the row
     *  WITHOUT a deref, so the per-candidate Java cost applies only to the
     *  overlap fraction. Valid rows are each <= ROW_CAP (510) entries and —
     *  since L41b — a narrow box stages up to 6x6 = 36 rows (the raw 4x4
     *  plus the 1-cell single-homing expansion), so 18432 = 36x510 + slack
     *  is the belt; the EMA still prices the bucket and flips to the mirror
     *  when the row scan loses. The old 8192 (4x4 sizing) refused the
     *  expanded dense windows wholesale (the mobs legs: rowBig exploded to
     *  ~11k refusals, the row path's serving collapsed). Wide shapes
     *  (targeting/sensor scans 7-35 blocks) keep the flat 512 cap — their
     *  window pass fraction is near 100%, so the scan cost there is
     *  per-CANDIDATE, not per-ENTRY. */
    private static final int ROW_TOTAL_CAP_NARROW = 18432;

    // Row-path diagnostics: hits and every distinct miss reason.
    public static volatile long rowFast = 0;
    public static volatile long rowMulti = 0, rowNoRow = 0, rowKey = 0, rowStale = 0;
    public static volatile long rowPending = 0, rowBig = 0, rowNoPub = 0, rowOv = 0;
    public static volatile long rowEmpty = 0, rowHomeMiss = 0, rowStaleCand = 0;
    public static volatile long rowWinSkip = 0; // L4c: candidates skipped by the packed x-window
    public static volatile long mViewRefuse = 0; // L44: native queries refused (view-exactness)
    public static volatile long rowNarrow = 0;  // L35: narrow-shape serves (raised cap)

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
    // L15b: hot-path counters are PLAIN longs — a volatile add per call cost
    // 1-2ms/tick on items-100k (100k merge checks); the JNI verifier and the
    // e2e reader tolerate eventual visibility, and these are write-only from
    // the single server thread.
    public static long mergeChecks = 0, mergeSelf = 0, mergeFast = 0;
    public static volatile long mergeMirror = 0, mergeCand = 0;
    /** L8 parity telemetry: successful tryToMerge operations (either side
     *  discarded). Entity counts can diverge legitimately through merge
     *  ORDER; the merge RATE + mass conservation (bench samples) decide
     *  whether the fast path's dynamics match vanilla. */
    public static volatile long mergeOps = 0;

    private static volatile java.lang.invoke.MethodHandle MH_TRY_MERGE;
    private static volatile java.lang.invoke.MethodHandle MH_IS_MERGABLE;
    private static volatile java.lang.invoke.MethodHandle MH_MERGEWN;
    private static volatile boolean MH_FAILED = false;
    private static volatile boolean MWN_FAILED = false;

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

    /** Lazy private-access handle: ItemEntity.mergeWithNeighbours()V — the
     *  EXACT vanilla merge body, run when the fast scan refuses (index
     *  unsure) or when the bisect knob turns the fast path off. Without it
     *  the flush's return-value-ignored call DROPPED every refused merge:
     *  104,383 dropped merges in one spread leg (run 40719262186,
     *  mirror=104383) — each refusal meant vanilla would have merged and
     *  the ON leg silently did not. */
    private static synchronized boolean initMergeWn(Object sample) {
        if (MWN_FAILED) return false;
        if (MH_MERGEWN != null) return true;
        try {
            Class<?> ie = sample.getClass();
            java.lang.reflect.Method mw = ie.getDeclaredMethod("mergeWithNeighbours");
            mw.setAccessible(true);
            MH_MERGEWN = java.lang.invoke.MethodHandles.lookup().unreflect(mw)
                    .asType(java.lang.invoke.MethodType.methodType(void.class, ie));
            return true;
        } catch (Throwable t) {
            MWN_FAILED = true;
            lastError = System.nanoTime();
            lastErrorText = "mergeWn: " + t;
            return false;
        }
    }

    /** Runs the exact vanilla merge body (mergeWithNeighbours). Returns
     *  false only if the handle could not be bound (the merge is then
     *  dropped, as before — the kill-switch counters will show it). */
    public static boolean runVanillaMerge(ItemEntity e) {
        try {
            if (MH_MERGEWN == null && !initMergeWn(e)) return false;
            MH_MERGEWN.invokeExact(e);
            return true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "runVanillaMerge: " + t;
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

    // === L44: the merge-scan VIEW (the pre-body boxes of resync entries) ===
    // Vanilla runs each item's merge scan DURING its body: entities whose
    // bodies FOLLOW the scanner's are seen at their PRE-move boxes. The
    // flush runs AFTER the whole entity phase, so a resync (vanilla-routed)
    // item's LIVE box is already POST-move — one physics step past the
    // vanilla view. For falling young items that step is 0.1-0.3 blocks and
    // the merge box's 0.25-tall Y window makes it decisive (items t=159:
    // 1827's spawn box y=-58.997 WAS inside 971's scan window, its post-body
    // y=-58.84 is 0.153 above it — exact numbers, run 41083916822 PTRACE).
    // The batch captures each resync entry's PRE-body state at itemSkip;
    // pass 1 overwrites IB_CD with the POST-body state for the tick
    // compute. IB_PRE keeps the capture; the apply pass exposes it to the
    // query paths as the VIEW for scanners positioned before that entry.
    private static double[] IB_PRE = new double[1024 * 6];
    private static final int[] VIEW_IDS = new int[256];
    private static final double[] VIEW_BOX = new double[256 * 6];
    private static final int[] VIEW_KIDX = new int[256];
    private static int viewN = 0;
    private static final java.util.HashMap<Integer, Integer> VIEW_MAP =
            new java.util.HashMap<>();
    private static int MS_KIDX = 0; // the current scanner's batch index

    /** The merge-scan view box for a candidate: the PRE-body box when the
     *  candidate is a resync entry whose body runs AFTER the scanner in the
     *  entity order, else the live box (vanilla's per-body timing). */
    private static AABB viewBox(int cid, AABB live) {
        Integer vi = VIEW_MAP.get(cid);
        if (vi == null || VIEW_KIDX[vi] <= MS_KIDX) return live;
        return new AABB(VIEW_BOX[vi * 6], VIEW_BOX[vi * 6 + 1], VIEW_BOX[vi * 6 + 2],
                VIEW_BOX[vi * 6 + 3], VIEW_BOX[vi * 6 + 4], VIEW_BOX[vi * 6 + 5]);
    }

    /** True when the candidate is an AFTER-scanner view entry (its live box
     *  is one physics step past the vanilla view — the window must deref it
     *  and the home re-check must not drop it). */
    private static boolean isViewAfter(int cid) {
        if (!IN_ITEM_APPLY) return false; // the view is apply-pass-only state
        Integer vi = VIEW_MAP.get(cid);
        return vi != null && VIEW_KIDX[vi] > MS_KIDX;
    }

    /** True when any AFTER-scanner view entry's PRE box intersects the query
     *  box: the native resolve (index boxes = POST-move) would answer with
     *  the wrong view, so the caller must not trust it. */
    private static boolean viewTouches(AABB box) {
        if (!IN_ITEM_APPLY) return false; // the view is apply-pass-only state
        for (int v = 0; v < viewN; v++) {
            if (VIEW_KIDX[v] <= MS_KIDX) continue;
            if (VIEW_BOX[v * 6] <= box.maxX && VIEW_BOX[v * 6 + 3] >= box.minX
                    && VIEW_BOX[v * 6 + 1] <= box.maxY && VIEW_BOX[v * 6 + 4] >= box.minY
                    && VIEW_BOX[v * 6 + 2] <= box.maxZ && VIEW_BOX[v * 6 + 5] >= box.minZ) {
                return true;
            }
        }
        return false;
    }

    private static boolean mergePredTest(Entity o) {
        if (!(o instanceof ItemEntity)) return false;
        ItemEntity ie = (ItemEntity) o;
        if (ie == MERGE_SELF) return false;
        if (MERGP_ON) return isMergableMirror(ie);
        return isMergableFast(ie);
    }

    // ---- L39 mirror merge predicate --------------------------------------
    // Byte-exact port of ItemEntity.isMergable (26.3):
    //   isAlive() && pickupDelay != 32767 && age != -32768
    //   && age < despawnRate && item.getCount() < item.getMaxStackSize()
    // with the three DEEP reads served by Unsafe hops instead of the method
    // chain: item = entityData.items[IDX_ITEM].value (the synched-data walk
    // SynchedEntityData.getItem was 2.91% of the items profile),
    // despawnRate/count by field offset. getMaxStackSize is pre-resolved
    // (L41) when provably exact — see mergeMaxStack.
    // The predicate runs per candidate per merge scan (~2500 scans/tick at
    // items-100k) — the L29 pattern applied to the merge family.
    public static volatile boolean MERGP_ON = false;
    private static volatile long O_IE_DESPAWN = -1, O_STACK_COUNT = -1;
    private static volatile int IDX_ITEM = -1;

    /** Resolves the three mirror offsets (ItemEntity.despawnRate,
     *  ItemStack.count, the DATA_ITEM accessor id). Runs once at arm. */
    public static void initMergeMirror() {
        try {
            java.lang.reflect.Field f;
            f = ItemEntity.class.getDeclaredField("despawnRate");
            f.setAccessible(true);
            O_IE_DESPAWN = U.objectFieldOffset(f);
            f = net.minecraft.world.item.ItemStack.class.getDeclaredField("count");
            f.setAccessible(true);
            O_STACK_COUNT = U.objectFieldOffset(f);
            f = ItemEntity.class.getDeclaredField("DATA_ITEM");
            f.setAccessible(true);
            Object fa = U.getObject(U.staticFieldBase(f), U.staticFieldOffset(f));
            if (fa instanceof net.minecraft.network.syncher.EntityDataAccessor) {
                IDX_ITEM = ((net.minecraft.network.syncher.EntityDataAccessor<?>) fa).id();
            }
            MERGP_ON = O_IE_DESPAWN >= 0 && O_STACK_COUNT >= 0 && IDX_ITEM >= 0
                    && !"0".equals(System.getenv("CE_MERGE_MIRROR"));
            initMergeMaxStack();
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initMergeMirror: " + t;
            MERGP_ON = false;
        }
    }

    // ---- L41 merge max-stack pre-resolution -------------------------------
    // ItemInstance.getMaxStackSize() (26.3) = getOrDefault(MAX_STACK_SIZE,1)
    // = getComponents().getOrDefault(...): a DataComponents walk per PAIR —
    // ItemInstance.getMaxStackSize was 8.29% of the items-100k profile.
    // The walk is EXACTLY prototype.get(type) whenever the stack's
    // PatchedDataComponentMap.patch is the shared fastutil EMPTY singleton:
    //   * the ctor assigns Reference2ObjectMaps.emptyMap() — javap-verified
    //     `getstatic EMPTY_MAP; areturn` (the global immutable instance);
    //   * getFromPatchAndPrototype: patch.get(type) == null ->
    //     prototype.get(type) — for the empty patch ALWAYS the prototype;
    //   * ensureMapOwnership() REPLACES the patch object (new
    //     Reference2ObjectArrayMap) BEFORE the first mutation — the
    //     singleton is never mutated in place, and copy() shares it
    //     only under copyOnWrite (the first set detaches). So
    //     patch == EMPTY_MAP is a STABLE, permanent property.
    // The prototype is the item-type's immutable default map: its
    // MAX_STACK_SIZE answer is a per-type constant cached by identity
    // (LAST_PROTO two-slot, server-thread confined like MERGE_SELF).
    // Any patched stack (NBT components, plugin mutation, a fromPatch
    // build with its own empty map) fails the singleton check and takes
    // the exact virtual walk. Bit-exact under every mutation pattern.
    // EMPTY-stack semantics stay vanilla: getComponents() returns
    // DataComponentMap.EMPTY for count==0 -> max 1 -> 0 < 1 -> mergeable;
    // the call site guards it before reaching here.
    private static volatile long O_STACK_COMPONENTS = -1, O_PCM_PATCH = -1, O_PCM_PROTO = -1;
    private static volatile Object EMPTY_PATCH;
    private static volatile Object MSSTYPE;
    private static Object LAST_PROTO;
    private static int LAST_PROTO_MAX;

    private static void initMergeMaxStack() {
        try {
            java.lang.reflect.Field f;
            f = net.minecraft.world.item.ItemStack.class.getDeclaredField("components");
            f.setAccessible(true);
            O_STACK_COMPONENTS = U.objectFieldOffset(f);
            Class<?> pcm = Class.forName(
                    "net.minecraft.core.component.PatchedDataComponentMap", false,
                    CeHooks.class.getClassLoader());
            f = pcm.getDeclaredField("patch");
            f.setAccessible(true);
            O_PCM_PATCH = U.objectFieldOffset(f);
            f = pcm.getDeclaredField("prototype");
            f.setAccessible(true);
            O_PCM_PROTO = U.objectFieldOffset(f);
            f = net.minecraft.core.component.DataComponents.class.getDeclaredField(
                    "MAX_STACK_SIZE");
            f.setAccessible(true);
            MSSTYPE = U.getObject(U.staticFieldBase(f), U.staticFieldOffset(f));
            EMPTY_PATCH = it.unimi.dsi.fastutil.objects.Reference2ObjectMaps.EMPTY_MAP;
        } catch (Throwable t) {
            // any miss leaves the offsets -1 -> mergeMaxStack always walks
            O_STACK_COMPONENTS = -1;
            lastError = System.nanoTime();
            lastErrorText = "initMergeMaxStack: " + t;
        }
    }

    /** The max-stack answer for a NON-EMPTY stack: the prototype constant
     *  when the patch map is the shared empty singleton, the exact walk
     *  otherwise. */
    private static int mergeMaxStack(Object st) {
        long o1 = O_STACK_COMPONENTS, o2 = O_PCM_PATCH, o3 = O_PCM_PROTO;
        Object mst = MSSTYPE;
        if (o1 >= 0 && o2 >= 0 && o3 >= 0 && mst != null) {
            Object map = U.getObject(st, o1);
            if (map != null) {
                Object patch = U.getObject(map, o2);
                if (patch == EMPTY_PATCH) {
                    Object proto = U.getObject(map, o3);
                    if (proto == LAST_PROTO) return LAST_PROTO_MAX;
                    try {
                        Object v = ((net.minecraft.core.component.DataComponentMap) proto)
                                .get((net.minecraft.core.component.DataComponentType<?>) mst);
                        Integer iv = (Integer) v;
                        LAST_PROTO_MAX = iv == null ? 1 : iv;
                        LAST_PROTO = proto;
                        return LAST_PROTO_MAX;
                    } catch (Throwable quiet) {
                        // fall through to the exact walk
                    }
                }
            }
        }
        return ((net.minecraft.world.item.ItemStack) st).getMaxStackSize();
    }

    @SuppressWarnings("unchecked")
    private static boolean isMergableMirror(ItemEntity e) {
        if (e.isRemoved()) return false;                 // Entity.isAlive
        if (e.pickupDelay == 32767 || e.age == -32768) return false;
        int despawn = U.getInt(e, O_IE_DESPAWN);
        if (e.age >= despawn) return false;
        // item = entityData.items[IDX_ITEM].value — the L29 3-hop pattern
        Object ed = U.getObject(e, O_ED);
        if (ed == null) return isMergableFast(e);        // belt: pre-init data
        Object[] items = (Object[]) U.getObject(ed, O_ED_ITEMS);
        if (items == null || IDX_ITEM >= items.length) return isMergableFast(e);
        Object di = items[IDX_ITEM];
        if (di == null) return isMergableFast(e);
        Object st = U.getObject(di, O_DI_VALUE);
        if (!(st instanceof net.minecraft.world.item.ItemStack stack)) {
            return isMergableFast(e);
        }
        int count = U.getInt(st, O_STACK_COUNT);
        if (count == 0) return true;   // EMPTY-map semantics: max 1, 0 < 1
        return count < mergeMaxStack(st);
    }

    private static final Predicate<Entity> MERGE_PRED = CeHooks::mergePredTest;

    // MSQ — the merge-scan query telemetry at PH_DUMP ticks (CUR_TICK
    // space): the PRE-PREDICATE candidate ids of MERGE_SELF's scan, the
    // path that served it (row/native/mirror), and the raw candidate
    // count the index returned. A vanilla scan that finds a partner
    // while the module's scan prints cands=0 names the exact query-path
    // false negative (run 40889274763: MS t=475 self=971 n=0 — vanilla's
    // scan of 971 found 1873, the tie absorbed 971, the leg diverged).
    private static final int[] MSQ_IDS = new int[256];
    private static int msqN = 0;
    private static boolean MSQ_COLLECT = false;
    private static String msqPath = null;
    private static long msqWin0 = 0, msqHome0 = 0, msqStale0 = 0;
    // Per-scan diagnostics set by the CALLER (server-thread confined):
    // MS_SRC 1 = the native-flush apply loop (the Rust cadence decided),
    // 0 = the vanilla-body guard (vanilla's own gate opened). MS_MV 1 = the
    // Rust cadence saw a floor change (rate 2), 2 = same floor (rate 40),
    // 0 = unknown (guard path). Both are read+cleared by the MS print.
    private static int MS_SRC = 0;
    private static int MS_MV = 0;
    // L41d marginExtra samples: the LIVE boxes of the first 8 rejects (the
    // index box lives in the Rust mirror — the Rust side can be extended to
    // dump its side; the live boxes alone already show the pile's spread).
    private static final double[] msqRej = new double[192];
    private static final int[] msqRejIds = new int[32];
    private static int msqRejN = 0;
    // L44b: the window-skipped and home-missed candidate ids at MSQ scans —
    // a merge partner that never reached the box test names itself here.
    private static final int[] msqSkipId = new int[32];
    private static final int[] msqSkipLo = new int[32];
    private static final int[] msqSkipHi = new int[32];
    private static final int[] msqSkipRow = new int[32];
    private static int msqSkipN = 0;
    private static final int[] msqHmId = new int[32];
    private static int msqHmN = 0;
    // L41e: the staged rows' (cx, cz, n, skips) — up to 12 rows per MSQ
    private static final long[] msqRows = new long[48];
    private static int msqRowsN = 0;

    private static String msqIds() {
        StringBuilder sb = new StringBuilder(msqN * 6 + 2);
        sb.append('[');
        for (int i = 0; i < msqN && i < 48; i++) {
            sb.append(i == 0 ? "" : ",").append(MSQ_IDS[i]);
        }
        return sb.append(msqN > 48 ? ",...]" : "]").toString();
    }

    // L45: WHY did fastQuery refuse this scan? The mirror MSQ line used to
    // say only path=mirror rawN=0 — the row-refusal reason (multi / key /
    // stale / fresh / pending / big / view / notready / ...) was invisible,
    // and the items t=219 hunt stalled on exactly that (run 41110186252:
    // 6931 mirror rawN=0, reason unnameable). Set at every fastQuery /
    // rowQuery refusal point; read by the mergeScanNative mirror branch.
    public static volatile String fastRefuse = null;

    public static boolean mergeScanNative(ItemEntity e) {
        mergeChecks++;
        if (!MERGE_ON || !ACTIVE) return false;
        try {
            net.minecraft.world.level.Level w = e.level();
            if (!(w instanceof net.minecraft.server.level.ServerLevel)
                    || w.isClientSide()) {
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
            boolean msq = phDumpTickNow();
            if (msq) {
                msqN = 0;
                msqPath = null;
                if (bN > 0) {
                    StringBuilder pb = new StringBuilder(200);
                    int lim = Math.min(bN, 24);
                    for (int k = 0; k < lim; k++) {
                        pb.append(k == 0 ? "" : ",").append((int) (B_IDS[k] & 0xFFFF_FFFFL))
                          .append(B_IDS[k] < 0 ? "!" : (B_IDS[k] & ROW_NEUTRAL) != 0 ? "~" : "+");
                    }
                    System.out.println("[ce-baseline] MSP t=" + CUR_TICK
                            + " self=" + e.getId() + " bN=" + bN
                            + " ids=" + pb + (bN > lim ? ",..." : ""));
                }
                msqWin0 = rowWinSkip;
                msqHome0 = rowHomeMiss;
                msqStale0 = rowStaleCand;
                msqRejN = 0;
                msqRowsN = 0;
                msqSkipN = 0;
                msqHmN = 0;
                MSQ_COLLECT = true;
            }
            List<Entity> cands;
            // L46: the depth guard spans the query AND the consume loop — the
            // scratch answer list must stay stable while mergeScanNative
            // iterates it (a merge body that queries reentrantly lands at
            // depth 2 and allocates its own list).
            fqScratchDepth++;
            try {
                try {
                    if (msq) {
                        System.out.println("[ce-baseline] QB t=" + CUR_TICK
                                + " self=" + e.getId()
                                + " q=[" + box.minX + "," + box.maxX
                                + " " + box.minY + "," + box.maxY
                                + " " + box.minZ + "," + box.maxZ + "]"
                                + " r=" + r);
                    }
                    cands = fastQuery(sl, e, box, MERGE_PRED, true);
                } finally {
                    MSQ_COLLECT = false;
                    MERGE_SELF = null;
                }
                if (cands == null) {
                    mergeMirror++;
                    if (msq) {
                        msqPath = msqPath == null ? "mirror" : msqPath;
                        System.out.println("[ce-baseline] MSQ t=" + CUR_TICK
                                + " self=" + e.getId() + " path=mirror"
                                + " rawN=" + msqN + " ids=" + msqIds()
                                + " refuse=" + fastRefuse);
                    }
                    return false; // index unsure — vanilla body
                }
                mergeFast++;
                mergeCand += cands.size();
            // MS: the scan's candidate list at PH_DUMP ticks — the parity
            // diff names the exact candidate-order divergence (vanilla's
            // first candidate kills the ticking item at equal counts, so
            // a different first candidate flips the survivor ids).
            if (phDumpTickNow()) {
                StringBuilder msb = new StringBuilder(96);
                int mv = MS_MV;
                MS_MV = 0;
                int src = MS_SRC;
                MS_SRC = 0;
                msb.append("MS t=").append(CUR_TICK)
                        .append(" self=").append(e.getId()).append(" n=").append(cands.size())
                        .append(" tc=").append(e.tickCount)
                        .append(" mv=").append(mv)
                        .append(" src=").append(src);
                for (int k = 0; k < Math.min(cands.size(), 24); k++) {
                    msb.append(' ').append(cands.get(k).getId());
                }
                System.out.println("[ce-baseline] " + msb);
                System.out.println("[ce-baseline] MSQ t=" + CUR_TICK
                        + " self=" + e.getId()
                        + " path=" + (msqPath == null ? "row" : msqPath)
                        + " rawN=" + msqN + " ids=" + msqIds()
                        + " win=" + (rowWinSkip - msqWin0)
                        + " home=" + (rowHomeMiss - msqHome0)
                        + " stale=" + (rowStaleCand - msqStale0));
                if (msqRejN > 0) {
                    StringBuilder rb = new StringBuilder(160);
                    for (int k = 0; k < msqRejN; k++) {
                        rb.append(msqRejIds[k]).append(":[")
                                .append((float) msqRej[k * 6]).append(',')
                                .append((float) msqRej[k * 6 + 2]).append(',')
                                .append((float) msqRej[k * 6 + 4]).append(']');
                    }
                    System.out.println("[ce-baseline] MSR t=" + CUR_TICK
                            + " self=" + e.getId() + " rej=" + msqRejN
                            + " liveMin=" + rb);
                }

                if (msqHmN > 0) {
                    StringBuilder sb4 = new StringBuilder(160);
                    for (int k = 0; k < msqHmN; k++) {
                        sb4.append(k == 0 ? "" : ",").append(msqHmId[k]);
                    }
                    System.out.println("[ce-baseline] MSH t=" + CUR_TICK
                            + " self=" + e.getId() + " homeMiss=" + msqHmN
                            + " ids=" + sb4);
                }
                if (msqRowsN > 0) {
                    StringBuilder sb2 = new StringBuilder(200);
                    for (int k = 0; k < msqRowsN && k < 12; k++) {
                        sb2.append(k == 0 ? "[" : ",")
                                .append('(').append(msqRows[k * 4])
                                .append(',').append(msqRows[k * 4 + 1])
                                .append(",n=").append(msqRows[k * 4 + 2])
                                .append(')');
                    }
                    System.out.println("[ce-baseline] MSR t=" + CUR_TICK
                            + " self=" + e.getId() + " rows=" + sb2.append(']'));
                }
            }
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
                if (e.isRemoved() | other.isRemoved()) mergeOps++;
                if (e.isRemoved()) return true; // absorbed — vanilla stops
            }
            return true;
            } finally {
                fqScratchDepth--;
            }
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
                    || level.isClientSide()) {
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
                + " marginExtra=" + marginExtra
                + " arenaMiss=" + arenaMiss + " stale=" + staleCand
                + " susp=" + suspends + " res=" + resumes + " resync=" + resyncEnts
                + " reprobe=" + fastProbes + "/" + mirrorProbes
                + " gate=" + gateStats()
                + " row=" + rowFast + "/" + rowMulti + "/" + rowStale + "/" + rowPending
                + "/" + rowNoRow + "/" + rowKey + "/" + rowBig + "/" + rowNoPub
                + "/" + rowOv + "/" + rowEmpty + "/" + rowHomeMiss + "/" + rowStaleCand
                + "/w" + rowWinSkip + "/n" + rowNarrow
                + " rest=" + restFast + "/" + restMarks + "/" + restWakes + "/" + restChecks + "/" + hurtProbes
                + " wakes=" + wakesDelta + "/" + wakesBox + "/" + wakesSelf
                + " restF=" + restFailGround + "/" + restFailDelay + "/" + restFailAge
                + "/" + restFailFluid + "/" + restFailVel + "/" + restFailChurn
                + " merge=" + mergeChecks + "/" + mergeSelf + "/" + mergeFast
                + "/" + mergeMirror + "/" + mergeCand + "/" + mergeOps
                + " offT=" + mOffThread
                + " dbgExp=" + Long.toHexString(dbgExpKey) + "/" + Long.toHexString(dbgGotKey)
                + "/" + dbgExpSlot + "/" + dbgMisses
                + " item=" + itemChecks + "/" + itemNative + "/" + itemResync
                + "/" + itemVanillaNext + "/" + itemAbort + "/" + itemFlushes
                + " disp=" + dispChecks + "/" + dispNative + "/" + dispInactive
                + "/" + dispPass + "/" + dispGate + "/r" + WORLD_REV
                + " rest=" + dispRest + "/" + restExits + "/" + restSectKeeps
                + " travel=" + travelChecks + "/" + travelQuiet + "/" + travelFail
                + " idle=" + navCalls + "/" + navIdleHits + "/" + moveCalls + "/" + moveIdleHits
                + "/" + lookCalls + "/" + lookIdleHits
                + " push=" + pushChecks + "/" + pushNative + "/" + pushMirror
                + "/" + pushCramming + "/" + pushSelfDelta
                + "/" + pushPairs + "/" + pushPairMiss
                + " tracker=" + trackerChecks + "/" + trackerFast + "/" + trackerVanilla
                + "/" + trackerErr
                + " ticklist=" + tickListChecks + "/" + tickListErr
                + " gs=" + gsFloorTicks + "/" + gsFloorRunning + "/" + gsFloorStarts
                + "/" + gsFloorStops + "/" + gsMirrorTicks + "/" + gsFloorCollide
                + " froz=" + frozenCalls + "/" + frozenGate + "/" + frozenWrites
                + " fd=" + frozenDirect + " cd=" + convDirect
                + " conv=" + convCalls + "/" + convGate + "/" + convWrites
                + " flag=" + flagCalls + "/" + flagGate + "/" + flagWrites
                + " delta=" + deltaCalls + "/" + deltaGate
                + " tt=" + ttCalls + "/" + ttGate
                + " ttl=" + ttlCalls + "/" + ttlFuse + "/" + ttlFuseChecks + "/" + (TTL_ON ? 1 : 0)
                + " eff=" + effCalls + "/" + effGate + "/" + (EFF_ON ? 1 : 0)
                + " mob=" + mobAiTicks + "/" + mobAiFallbacks + "/" + mobAiRequests
                + "/" + mobAiClassified + "/" + mobAiRefused + "/" + (MOB_AI_ON ? 1 : 0)
                + " oddJ=" + mobAiOddJava
                + " mobwhy=" + mobAiFallbackWhy + "/" + mobAiFallbackRound + "/"
                + mobAiFallbackReq + " " + mobAiFallbackMob + " refused=" + mobAiRefusedClass
                + " wclash=" + mWorldSlotClash
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

    /** Prefix of Entity.setDeltaMovement(Vec3) — the VALUE-CARRYING capture
     *  (L42). The 26.3 body assigns the ARGUMENT (an isFinite-gated field
     *  write), so the argument IS the post-write field value; the old
     *  field-read capture (probeDelta(Entity)) always batched the PRE-write
     *  value, which left the push mirror's delta permanently one write
     *  behind — for a standing mob the box event re-captured the post-move
     *  pre-tail (0, 0, 0) every tick (the tail's (0, -g*vf, 0) write was
     *  value-identical, the L20b gate skipped it AND its probe event), so
     *  the kernel's self writeback replaced the fresh vanilla travel result
     *  with (stale base + push): vy=0.0 ("the collision zeroed vy"), the
     *  un-frictioned vx — the exact mobs/spread first-move PH divergence
     *  (doc 40 addendum 5, attributed there to the travel classification).
     *  The DDD overload delegates to this one, so its own wake-only probe
     *  fires first and this fires second — the LAST event per entity wins
     *  at the drain and carries the new value. */
    public static void probeDelta(Entity e, net.minecraft.world.phys.Vec3 v) {
        if (v != null) {
            probeDeltaCommon(e, v.x, v.y, v.z);
        } else {
            // never observed (vanilla constructs the Vec3 before the call);
            // the wake must still fire, the batch must not
            probeDeltaCommon(e, Double.NaN, Double.NaN, Double.NaN);
        }
    }

    /** The shared capture body: the wake semantics are UNCHANGED (every
     *  real setDeltaMovement call wakes a resting item), only the batched
     *  values changed (the argument instead of the pre-write field). */
    private static void probeDeltaCommon(Entity e, double x, double y, double z) {
        try {
            // L7 first: an external velocity change on a dispatch-owned item
            // clears its native mode (the applier's own writebacks run inside
            // IN_ITEM_APPLY and do not trip this).
            l7ExternalMutation(e);
            if (e != null) {
                int m = e.getId() & AMASK;
                // L12 push mirror: keep the native delta fresh. Only living
                // entities are push-kernel state; the delta events of items
                // are the item core's business. Off-thread callers skip (the
                // next server-thread box event refreshes the mirror).
                if (ACTIVE && INDEXING && e instanceof net.minecraft.world.entity.LivingEntity
                        && SERVER_THREAD.get()) {
                    int pw = worldId(e.level());
                    if (pw >= 0 && ARENA[m] == e
                            // the vanilla body SKIPS non-finite writes (the
                            // Vec3 isFinite gate) — the field keeps the old
                            // value, the mirror must too. Unreachable with
                            // the L20b gate woven (it skips first); this is
                            // the gate-off exactness guard.
                            && Double.isFinite(x) && Double.isFinite(y)
                            && Double.isFinite(z)) {
                        if (dbN >= BATCH_CAP) {
                            flush();
                        }
                        DB_IDS[dbN] = ((pw & 0xFFFFL) << 32)
                                | (e.getId() & 0xFFFF_FFFFL);
                        DB_DV[dbN * 3] = x;
                        DB_DV[dbN * 3 + 1] = y;
                        DB_DV[dbN * 3 + 2] = z;
                        dbN++;
                        if (dbN >= DRAIN_CAP) flush();
                    }
                }
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

    /** Prefix of Entity.setDeltaMovement(DDD) — wake-only. The 26.3 DDD
     *  body DELEGATES to setDeltaMovement(Vec3) (javap-verified), so this
     *  fires on the outer frame and the value-carrying probe above fires on
     *  the inner one; the drain's last-event-wins keeps the mirror exact.
     *  No batch here: the pre-write field read is now worthless (the inner
     *  event carries the new value) and the double batch taxed the drain. */
    public static void probeDelta(Entity e) {
        try {
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
            if (ACTIVE && INDEXING && e != null && e.level() != null && !e.level().isClientSide()) {
                probesAdd++;
                ARENA[e.getId() & AMASK] = e;
                ordStamp(e); // L37: vanilla addEntity appends -> fresh seq
                batchUpsert(e.getId(), worldId(e.level()), e, e.getBoundingBox(), false);
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

    /** L38f: LivingEntity.die(DamageSource) prefix. javap-verified 26.3:
     *  hurtServer deducts hp through actuallyHurt BEFORE `this.die(source)`
     *  (the isDeadOrDying gate at the tail of the hurt flow) — at the
     *  prefix hp is ALREADY <= 0, so isAlive() is false and the freshly
     *  recomputed pushFlags clear PF_PUSHABLE. Without this probe the
     *  corpse (position frozen, zero box/delta events for the whole
     *  ~20-tick deathTime animation) stays pushable in the mirror while
     *  vanilla's getPushableEntities excludes it — the kernel's cramming
     *  audit then counts it (hits/nonPass) and applies hurtServer where
     *  vanilla does not: an extra hurt with ZERO RNG draw (cramming has
     *  no source position -> dealDefaultKnockback passes xd=zd=0, no
     *  Math.random, no velocity) and ZERO position impact — the exact
     *  PH-invisible hp divergence that failed the T382+ snapshot gate
     *  (doc 40). The upsert rides the pending batch; the next
     *  nativePush of THIS tick drains it (flushIfPending / drain_direct
     *  at the kernel entry), so the within-tick pusher order sees the
     *  corpse exactly when vanilla's fresh per-pusher list does. */
    public static void probeDie(Entity e) {
        try {
            if (ACTIVE && INDEXING && e != null
                    && e instanceof net.minecraft.world.entity.LivingEntity
                    && SERVER_THREAD.get()
                    && e.level() != null && !e.level().isClientSide()) {
                ARENA[e.getId() & AMASK] = e;
                batchUpsert(e.getId(), worldId(e.level()), e, e.getBoundingBox(), true);
            }
        } catch (Throwable t) {
            // uncertain -> the mirror stays as-is. The failure mode is the
            // PRE-fix staleness (a missed flag refresh can only LEAVE a
            // corpse in the kernel's view — an extra audit hit), never a
            // spurious removal: this probe may only shrink the view.
            lastError = System.nanoTime();
            lastErrorText = "probeDie: " + t;
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
            if (ACTIVE && INDEXING && e != null && newBox != null && e.level() != null && !e.level().isClientSide()) {
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
                // PARITY ORDER (doc 36b): a section-Y change WITHIN the cell
                // (y crossing a 16-block boundary) is a vanilla RE-APPEND —
                // the row's id ORDER changes even though the id LIST does
                // not, so it must NOT classify row-neutral (the row keeps
                // its old order until the next dirty republish).
                AABB oldBox = e.getBoundingBox();
                int curSy = e.moonrise$getSectionY();
                boolean syMoved = LAST_SY[e.getId() & AMASK] != curSy;
                LAST_SY[e.getId() & AMASK] = curSy;
                boolean rowNeutral = !syMoved && oldBox != null
                        && cellOfBox(oldBox) == cellOfBox(newBox);
                batchUpsert(e.getId(), worldId(e.level()), e, newBox, rowNeutral);
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
    // L12 push-state extension: per entry — flag bits, exact x/z, delta xyz,
    // direct vehicle id. Zero-filled for non-LivingEntity entries.
    private static final int[] B_PF = new int[BATCH_CAP];
    private static final double[] B_PXZ = new double[BATCH_CAP * 2];
    private static final double[] B_DV = new double[BATCH_CAP * 3];
    private static final int[] B_VID = new int[BATCH_CAP];
    // L37 vanilla-order keys: the clamped section Y + the insertion seq per
    // entry — the push kernel sorts cell candidates by (chunkZ, chunkX,
    // sectionY, seq) exactly like vanilla's EntityLookup scan (chunkZ/X come
    // from the mirrored px/pz at sort time).
    private static final int[] B_SY = new int[BATCH_CAP];
    private static final int[] B_SEQ = new int[BATCH_CAP];
    // L12 delta-only batch (probeDelta events without a box change)
    private static final long[] DB_IDS = new long[BATCH_CAP];
    private static final double[] DB_DV = new double[BATCH_CAP * 3];
    private static int dbN = 0;
    private static int bN = 0;
    private static final ThreadLocal<Boolean> SERVER_THREAD =
            ThreadLocal.withInitial(() -> "Server thread".equals(Thread.currentThread().getName()));

    private static void batchUpsert(int id, int wid, Entity e, AABB b, boolean rowNeutral) {
        if (!SERVER_THREAD.get()) {
            // off-thread: the push mirror is server-thread state; off-thread
            // spawns refresh it on the next server-thread box event (every
            // entity is box-probed at least once after spawn) — the exactness
            // contract is documented in docs/research/23.
            nativeBoxDirect(id, wid,
                    b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
            return;
        }
        B_IDS[bN] = ((wid & 0xFFFFL) << 32) | (id & 0xFFFF_FFFFL)
                | (rowNeutral ? ROW_NEUTRAL : 0L);
        int o = bN * 6;
        B_BOX[o] = b.minX; B_BOX[o + 1] = b.minY; B_BOX[o + 2] = b.minZ;
        B_BOX[o + 3] = b.maxX; B_BOX[o + 4] = b.maxY; B_BOX[o + 5] = b.maxZ;
        // PARITY ORDER (doc 36b): the exact (sectionY, insertion seq) stamps
        // travel with EVERY entry — items merge-scan through the same index
        // and their ties are order-sensitive (vanilla EntityLookup order:
        // chunkZ, chunkX, sectionY, seq). moonrise$getSectionY and the
        // ORD_SEQ stamp live on plain Entity, not just LivingEntity.
        B_SY[bN] = e.moonrise$getSectionY();
        B_SEQ[bN] = ORD_SEQ[e.getId() & AMASK];
        if (e instanceof net.minecraft.world.entity.LivingEntity le) {
            B_PF[bN] = pushFlags(le);
            B_PXZ[bN * 2] = le.getX();
            B_PXZ[bN * 2 + 1] = le.getZ();
            Vec3 dv = le.getDeltaMovement();
            B_DV[bN * 3] = dv.x; B_DV[bN * 3 + 1] = dv.y; B_DV[bN * 3 + 2] = dv.z;
            Entity veh = le.isPassenger() ? le.getVehicle() : null;
            B_VID[bN] = veh != null ? veh.getId() : 0;
        } else {
            B_PF[bN] = 0;
            B_PXZ[bN * 2] = 0.0;
            B_PXZ[bN * 2 + 1] = 0.0;
            B_DV[bN * 3] = 0.0; B_DV[bN * 3 + 1] = 0.0; B_DV[bN * 3 + 2] = 0.0;
            B_VID[bN] = 0;
        }
        bN++;
        if (bN >= DRAIN_CAP) flush();
    }

    /** L12: the pushableBy predicate's stateful slice, folded into flag bits
     *  (javap-verified against 26.3 LivingEntity.isPushable ->
     *  isCollidable(false): isAlwaysTicking false, ServerLevel,
     *  isPositionEntityTicking(blockPosition), isAlive, !isSpectator,
     *  collides, !onClimbable). The team-rule slice is NOT folded — any
     *  entity WITH a team routes the whole push vanilla (the guard checks).
     *  Rare state flips without a box event (sleeping villager, spectator
     *  flip) re-classify on the entity's next box probe; the deviation
     *  window is documented in docs/research/23 and is behaviorally inert
     *  (deltas of non-traveling entities never integrate into position). */
    private static int pushFlags(net.minecraft.world.entity.LivingEntity le) {
        // THROW-PROOF BY CONTRACT: the first A/B round lost the whole module
        // to `le.isSpectator()` NPE'ing on a MID-CONSTRUCTION ServerPlayer
        // (this.gameMode is null during early boot) — the probeBox catch
        // then flipped ACTIVE=false permanently and the leg measured a
        // vanilla-plus-probe-tax world. Every flag read lives inside one
        // try; any throwable just drops the entity from the push view.
        int f = 0;
        try {
            if (le instanceof net.minecraft.server.level.ServerPlayer) f |= PF_PLAYER;
            if (le.isPassenger()) f |= PF_PASSENGER;
            if (le.isVehicle()) f |= PF_VEHICLE;
            if (le.noPhysics) f |= PF_NOPHYSICS;
            if (le.isSpectator()) f |= PF_SPECTATOR;
            if (!le.isAlwaysTicking()
                    && le.level() instanceof net.minecraft.server.level.ServerLevel sl
                    && sl.isPositionEntityTicking(le.blockPosition())
                    && le.isAlive()
                    && !le.isSpectator()
                    && le.collides
                    && !le.onClimbable()) {
                f |= PF_PUSHABLE;
            }
        } catch (Throwable t) {
            // uncertain -> NOT pushable: the entity drops out of the push
            // view (a missed push candidate, never a spurious one). The
            // bits computed so far stay (they are plain field reads).
        }
        return f;
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
        if (bN > 0 || dbN > 0) {
            batchesFlushed++;
            batchEntries += bN;
            nativeFlush(bN, B_IDS, B_BOX, B_PF, B_PXZ, B_DV, B_VID, B_SY, B_SEQ,
                    dbN, DB_IDS, DB_DV);
            bN = 0;
            dbN = 0;
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
        // L19: the thread check moved UP from fastQuery — off-thread callers
        // (worldgen, async plugins) took the vanilla mirror anyway (fastQuery
        // refused them after the gate bookkeeping was already paid). This
        // both removes that dead bookkeeping and makes the widFast plain
        // statics below single-writer (server thread only).
        if (!SERVER_THREAD.get()) {
            return -1;
        }
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
    // PARITY ORDER: the last section-Y seen per id (probeBox) — a change
    // within the same home cell is a vanilla re-append and must force a
    // dirty republish (see the rowNeutral classification in probeBox).
    private static final int[] LAST_SY = new int[ARENA.length];

    // ------------------------------------------------------------------
    // L37 — VANILLA SCAN ORDER. The parity A/B (run 40529103630) proved the
    // fast path's candidate ORDER is behavior-visible: ItemEntity.tryToMerge
    // keeps the FIRST candidate in vanilla scan order, and the
    // maxEntityCollisions cap keeps the FIRST candidates — a different order
    // merges different items and caps different mobs, and the SHARED_RANDOM
    // stream never re-converges (mobs leg: 14 ZOMBIE-vs-CHICKEN id flips,
    // 8 extra entities; items leg: 143/95 different merge survivors).
    //
    // Vanilla EntityLookup.getEntities iterates regions (Z outer, X inner),
    // chunks (Z outer, X inner — equivalent to plain chunkZ-major order),
    // sections Y ascending, then each section's BasicEntityList in
    // INSERTION order (add appends; removal is an arraycopy shift, so
    // relative order of the survivors is preserved; moveEntity re-appends
    // on a real section change). Therefore the exact order key per entity
    // is (chunkZ, chunkX, sectionY, insertionSeq):
    //   - chunkZ/X and sectionY: the entity's own moonrise-tracked section
    //     fields (moonrise$getSectionZ/X/Y — public final, updated by the
    //     same moveEntity body we mirror);
    //   - insertionSeq: a global counter bumped at addEntity (probeAdd) and
    //     at moveEntity's real-change re-append (probeMoveSection). A global
    //     counter is exact: within one section the later insertion has the
    //     larger seq; cross-section order is decided by the (Z, X, Y) key
    //     first. Removals preserve relative order — no renumbering needed.
    // ------------------------------------------------------------------
    public static final int[] ORD_SEQ = new int[ARENA.length];
    private static final java.util.concurrent.atomic.AtomicInteger ORD_CNT =
            new java.util.concurrent.atomic.AtomicInteger(1);

    private static void ordStamp(Entity e) {
        ORD_SEQ[e.getId() & AMASK] = ORD_CNT.incrementAndGet();
    }

    /** Prefix of EntityLookup.moveEntity(Entity): the vanilla RE-APPEND
     *  point (removeEntity + addEntity on a real section change). onMove
     *  calls moveEntity on EVERY position change; at prefix time the
     *  entity's moonrise section fields still hold the OLD section while
     *  the position field is already new (setPosRaw assigned it before the
     *  callback) — so the exact change test (including the section-Y clamp
     *  the body applies) is computable without touching the body. */
    public static void probeMoveSection(Entity e) {
        try {
            if (!ACTIVE || !INDEXING || e == null) return;
            if (SECTION_STAMP_SCHEME != 0) return; // slices-append scheme owns the stamp
            net.minecraft.world.level.Level lvl = e.level();
            if (lvl == null || lvl.isClientSide()) return;
            int oz = e.moonrise$getSectionZ();
            int ox = e.moonrise$getSectionX();
            int oy = e.moonrise$getSectionY();
            int nx = (int) Math.floor(e.getX()) >> 4;
            int nz = (int) Math.floor(e.getZ()) >> 4;
            int nyr = (int) Math.floor(e.getY()) >> 4;
            int ny = nyr < lvl.getMinSectionY() ? lvl.getMinSectionY()
                    : nyr > lvl.getMaxSectionY() ? lvl.getMaxSectionY() : nyr;
            if (nx != ox || nz != oz || ny != oy) {
                ORD_SEQ[e.getId() & AMASK] = ORD_CNT.incrementAndGet();
            }
        } catch (Throwable t) {
            // never break movement: a missed stamp costs order fidelity on
            // one entity, a throw here would desync every world tick
        }
    }

    // The stamp-scheme selection (patch_lookup): 1 = the SLICES scheme — the
    // seq stamp + moonrise field pre-write happen in the ChunkEntitySlices.
    // addEntity PREFIX, which is the ONLY point where the live section
    // structure, the moonrise fields and the order key are simultaneously
    // consistent (between removeEntity and addEntity the entity is INVISIBLE
    // to vanilla scans, so the pre-written fields cannot desync anything);
    // 0 = the legacy moveEntity-PREFIX scheme — the stamp lands BEFORE the
    // actual re-append, and every query inside the remove+add+setSection
    // window sorts a (stale section, fresh seq) key that disagrees with the
    // live structure — THE residual parity divergence (run 40647817613:
    // 12/16500 merge flips, jockey lottery desync). The slices patch is
    // attempted first; a refusal falls back to 0 (exact-ish, never worse).
    // volatile (not final): the native side flips it to 0 when the slices
    // weave is refused (the moveEntity fallback must then own the stamps).
    public static volatile int SECTION_STAMP_SCHEME = 1;

    public static void setSectionStampScheme(int v) {
        SECTION_STAMP_SCHEME = v;
    }

    /** PARITY ORDER (doc 36b): the ChunkEntitySlices.addEntity PREFIX —
     *  the vanilla re-append point. The receiver slices' chunkX/chunkZ are
     *  the NEW chunk coords and `sy` the (already clamped) new section Y.
     *  We PRE-WRITE the moonrise section fields and bump the insertion seq
     *  HERE: the live structure still holds the entity in the old section
     *  (the body appends it), but between removeEntity and addEntity the
     *  entity is invisible to every vanilla scan — no observer can read the
     *  transient state. From this point on (new fields + fresh seq) the
     *  order key is consistent with the live structure FOREVER — the
     *  moveEntity-prefix window (remove+add+setSection, dozens of ops with
     *  allocations) is gone. Also pushes an immediate batchUpsert so the
     *  Rust index/published rows re-sync in the SAME tick. */
    public static void probeSectionAppend(Entity e, int sy) {
        try {
            if (!ACTIVE || !INDEXING || e == null) return;
            net.minecraft.world.level.Level lvl = e.level();
            if (lvl == null || lvl.isClientSide()) return;
            // moveEntity computes the target chunk from blockPosition right
            // before calling addEntity, and chunk-load paths add entities at
            // their own position — deriving the new chunk coords from the
            // entity is exact. `sy` is the ALREADY clamped section Y the
            // slices body receives.
            int nx = (int) Math.floor(e.getX()) >> 4;
            int nz = (int) Math.floor(e.getZ()) >> 4;
            e.moonrise$setSectionX(nx);
            e.moonrise$setSectionY(sy);
            e.moonrise$setSectionZ(nz);
            ORD_SEQ[e.getId() & AMASK] = ORD_CNT.incrementAndGet();
            LAST_SY[e.getId() & AMASK] = sy;
            if (lvl instanceof net.minecraft.server.level.ServerLevel) {
                AABB b = e.getBoundingBox();
                if (b != null) {
                    int wid = worldId(lvl);
                    if (wid >= 0) {
                        batchUpsert(e.getId(), wid, e, b, false);
                    }
                }
            }
        } catch (Throwable t) {
            // never break movement — same reasoning as probeMoveSection
        }
    }

    // Sort scratch (server-thread confined — fastQuery/rowQuery callers).
    private static Entity[] ORD_E = new Entity[0];
    private static long[] ORD_KA = new long[0];
    private static long[] ORD_KB = new long[0];

    /** L38 — the collide fast path (CE_COLLIDE=0 disables the weave).
     *  Byte-exact port of LivingEntity.canCollideWithBukkit (26.3):
     *  `isPushable() && collides != collidableExemptions.contains(uuid)` —
     *  with the EMPTY-SET fast path. A default Paper server keeps
     *  collidableExemptions empty (Bukkit-API opt-in), and HashSet.contains
     *  pays a full UUID hash + table probe for an answer that is provably
     *  FALSE for an empty set — EntitySelector.pushableBy evaluates this
     *  per candidate per push/collision query, 7.1% of the mobs-25k-mod
     *  profile (the single largest controllable leaf). isPushable() stays a
     *  real virtual call; the exempt branch on a NON-empty set is exactly
     *  vanilla (same hash, same probe, same result). */
    public static boolean canCollideWithBukkit$(net.minecraft.world.entity.LivingEntity le,
                                                net.minecraft.world.entity.Entity other) {
        if (!le.isPushable()) {
            return false;
        }
        java.util.Set<java.util.UUID> ex = le.collidableExemptions;
        boolean exempt = ex != null && !ex.isEmpty() && ex.contains(other.getUUID());
        return le.collides != exempt;
    }

    /** The two-long vanilla order key: ka = (chunkZ biased 22b) << 22 |
     *  (chunkX biased 22b); kb = (sectionY biased 22b) << 40 | seq (32b). */
    private static void ordKey(Entity e, int i) {
        int cz = e.moonrise$getSectionZ();
        int cx = e.moonrise$getSectionX();
        int sy = e.moonrise$getSectionY();
        int seq = ORD_SEQ[e.getId() & AMASK];
        ORD_KA[i] = ((((long) cz + 2097152L) & 0x3FFFFFL) << 22)
                | (((long) cx + 2097152L) & 0x3FFFFFL);
        ORD_KB[i] = ((((long) sy + 2097152L) & 0x3FFFFFL) << 40)
                | ((long) seq & 0xFFFFFFFFL);
    }

    private static boolean ordLess(int i, long ta, long tb) {
        long a = ORD_KA[i];
        return a > ta || (a == ta && ORD_KB[i] > tb);
    }

    /** L37: stamp every entity that never passed probeAdd/probeMoveSection
     *  (added before the arm — world load during boot) in the EXACT vanilla
     *  scan order. Per-REGION boxes built from the Bukkit loaded-chunk list:
     *  a full-world box would make EntityLookup.getEntities iterate ~3.5
     *  BILLION region coordinates (observed: the arm thread hung forever —
     *  no ARMED line at all). Within one region the walk IS the vanilla
     *  order (chunks Z-major -> sections Y -> insertion), and cross-section
     *  order needs no stamps (the live moonrise section fields decide the
     *  key) — so region-level coarseness is exact. The arm thread may read
     *  slices concurrently with the tick: BasicEntityList is an index-scanned
     *  plain array (SW mr-designed), worst case a missed stamp that the
     *  entity's next add/move probe heals. Also backfills the ARENA. */
    private static void ordBackfillWorlds() {
        try {
            Class<?> bukkit = Class.forName("org.bukkit.Bukkit");
            java.util.List<?> worlds = (java.util.List<?>) bukkit
                    .getMethod("getWorlds").invoke(null);
            Class<?> craftWorld = Class.forName("org.bukkit.craftbukkit.CraftWorld");
            java.lang.reflect.Method getHandle = craftWorld.getMethod("getHandle");
            java.lang.reflect.Method getLookup = net.minecraft.server.level.ServerLevel.class
                    .getMethod("moonrise$getEntityLookup");
            Class<?> lookupClass = Class.forName(
                    "ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup");
            java.lang.reflect.Method getEnts = null;
            for (java.lang.reflect.Method m : lookupClass.getMethods()) {
                if (m.getName().equals("getEntities")
                        && m.getParameterCount() == 4
                        && m.getParameterTypes()[0] == Entity.class) {
                    getEnts = m;
                }
            }
            if (getEnts == null) {
                lastErrorText = "ordBackfill: EntityLookup.getEntities(Entity,AABB,List,Pred) not found";
                return;
            }
            java.lang.reflect.Method getChunks = Class.forName("org.bukkit.World")
                    .getMethod("getLoadedChunks");
            for (Object w : worlds) {
                Object sl = getHandle.invoke(w);
                if (!(sl instanceof net.minecraft.server.level.ServerLevel serverLevel)) {
                    continue;
                }
                Object lookup = getLookup.invoke(serverLevel);
                Object[] chunks = (Object[]) getChunks.invoke(w);
                // region key -> [minCx, maxCx, minCz, maxCz]
                java.util.HashMap<Long, int[]> regions = new java.util.HashMap<>();
                for (Object c : chunks) {
                    int cx = (Integer) c.getClass().getMethod("getX").invoke(c);
                    int cz = (Integer) c.getClass().getMethod("getZ").invoke(c);
                    long rk = (long) (cx >> 5) << 32 | (long) ((cz >> 5) & 0xFFFFFFFFL);
                    int[] b = regions.get(rk);
                    if (b == null) {
                        regions.put(rk, new int[]{cx, cx, cz, cz});
                    } else {
                        if (cx < b[0]) b[0] = cx;
                        if (cx > b[1]) b[1] = cx;
                        if (cz < b[2]) b[2] = cz;
                        if (cz > b[3]) b[3] = cz;
                    }
                }
                for (int[] b : regions.values()) {
                    AABB box = new AABB(
                            (double) (b[0] << 4), -2048.0, (double) (b[2] << 4),
                            (double) ((b[1] + 1) << 4), 2048.0, (double) ((b[3] + 1) << 4));
                    List<Entity> all = new ArrayList<>();
                    getEnts.invoke(lookup, null, box, all, null);
                    for (int i = 0; i < all.size(); i++) {
                        Entity en = all.get(i);
                        ARENA[en.getId() & AMASK] = en;
                        ordStamp(en);
                    }
                }
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "ordBackfillWorlds: " + t;
        }
    }

    /** Sort `out` into the exact vanilla EntityLookup.getEntities order.
     *  Insertion sort for the common sizes (push/merge/sensing scans are
     *  tens of candidates); comparator fallback for the rare big scans. */
    private static void ordSortVanilla(java.util.ArrayList<Entity> out) {
        int n = out.size();
        if (n < 2) return;
        if (n <= 512) {
            if (ORD_E.length < n) {
                ORD_E = new Entity[n];
                ORD_KA = new long[n];
                ORD_KB = new long[n];
            }
            for (int i = 0; i < n; i++) {
                Entity en = out.get(i);
                ORD_E[i] = en;
                ordKey(en, i);
            }
            for (int i = 1; i < n; i++) {
                Entity te = ORD_E[i];
                long ta = ORD_KA[i];
                long tb = ORD_KB[i];
                int j = i - 1;
                while (j >= 0 && ordLess(j, ta, tb)) {
                    ORD_E[j + 1] = ORD_E[j];
                    ORD_KA[j + 1] = ORD_KA[j];
                    ORD_KB[j + 1] = ORD_KB[j];
                    j--;
                }
                ORD_E[j + 1] = te;
                ORD_KA[j + 1] = ta;
                ORD_KB[j + 1] = tb;
            }
            for (int i = 0; i < n; i++) out.set(i, ORD_E[i]);
        } else {
            Entity[] arr = out.toArray(new Entity[0]);
            java.util.Arrays.sort(arr, (a, b) -> {
                long cza = a.moonrise$getSectionZ();
                long czb = b.moonrise$getSectionZ();
                if (cza != czb) return Long.compare(cza, czb);
                long cxa = a.moonrise$getSectionX();
                long cxb = b.moonrise$getSectionX();
                if (cxa != cxb) return Long.compare(cxa, cxb);
                long sya = a.moonrise$getSectionY();
                long syb = b.moonrise$getSectionY();
                if (sya != syb) return Long.compare(sya, syb);
                return Integer.compare(ORD_SEQ[a.getId() & AMASK], ORD_SEQ[b.getId() & AMASK]);
            });
            for (int i = 0; i < n; i++) out.set(i, arr[i]);
        }
    }

    // ------------------------------------------------------------------
    // Fast query: flush pending probes, Rust exact query -> resolve ids.
    // Returns null => caller takes the mirror path.
    // ------------------------------------------------------------------
    private static final ThreadLocal<long[]> QBUF =
            ThreadLocal.withInitial(() -> new long[1024]);

    // L46 (doc 41 move 2): the merge-scan caller consumes its candidate list
    // synchronously inside mergeScanNative — a fresh ArrayList per scan
    // (~32k scans/tick on items-100k) churned TLAB garbage that showed up as
    // allocator + GC pressure inside fastQuery's 13.96% self-time bucket.
    // The scratch list is server-thread-confined (fastQuery already refuses
    // off-thread callers) and guarded by a depth counter: a REENTRANT
    // fastQuery inside the consume loop (a merge body that queries) must
    // never see its own answer list cleared under it — depth > 1 allocates
    // normally. The general getEntities hook keeps allocating (callers may
    // retain the list).
    private static final ArrayList<Entity> FQ_SCRATCH = new ArrayList<>(512);
    private static int fqScratchDepth = 0;
    // L46 parity knob: CE_FQ_SCRATCH=0 restores the allocate-per-scan answer
    // list (the bisect handle if a red gate pins anything to the scratch).
    private static final boolean FQ_SCRATCH_ON =
            !"0".equals(System.getenv("CE_FQ_SCRATCH"));

    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, Entity source,
            AABB box, Predicate<? super Entity> predicate) {
        return fastQuery(level, source, box, predicate, false);
    }

    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, Entity source,
            AABB box, Predicate<? super Entity> predicate, boolean reuseScratch) {
        fastRefuse = null;
        // L46: only the OUTERMOST guarded caller may take the scratch —
        // a reentrant query inside a consume loop allocates (see FQ_SCRATCH).
        final boolean useScratch = FQ_SCRATCH_ON && reuseScratch && fqScratchDepth == 1;
        if (!(level instanceof net.minecraft.server.level.ServerLevel sl)) {
            mNotServerLevel++;
            fastRefuse = "notserverlevel";
            return null;
        }
        // The fast path touches server-thread-only state (batch arrays, gate
        // EMAs, published rows). Off-thread callers (async plugin queries)
        // take the vanilla mirror — exact parity for them, and it removes a
        // pre-existing batch-array race.
        if (!SERVER_THREAD.get()) {
            mOffThread++;
            fastRefuse = "offthread";
            return null;
        }
        int wid = worldId(sl);
        if (wid < 0) {
            mBadWorld++;
            fastRefuse = "badworld";
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
                List<Entity> out = rowQuery(sl, wid, source, box, predicate, useScratch);
                if (out != null) {
                    msqPath = "row";
                    return out;
                }
            }
        } else if (PUBLISH_SEQ == 0) {
            rowNoPub++;
        }
        msqPath = "native";
        // L44: the native resolve filters against the INDEX boxes (POST-move
        // for resync entries). When any AFTER-scanner view entry's PRE box
        // touches the query, that answer is one physics step past vanilla's
        // per-body view — refuse instead (the row path served it above; a
        // refusal here falls to the mirror, which is equally view-stale but
        // rare: it needs a row-path refusal AND a fresh spawn in range).
        if (viewTouches(box)) {
            mViewRefuse++;
            // keep the ROW reason: the chain "pending+view" names BOTH the
            // row refusal and the native view refusal in one print (the
            // first L45 run overwrote it and the items t=219 row reason
            // stayed unnameable).
            fastRefuse = fastRefuse == null ? "view" : fastRefuse + "+view";
            return null;
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
        int n;
        java.nio.ByteBuffer dbb = IDS_BUF;
        if (dbb != null && IDS_BUF_CAP > 0) {
            // DIRECT OUT BUFFER path (doc 21): one JNI crossing, zero per-
            // query array ops; ids are read back via intrinsified getLong.
            n = queryNative2(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                    bn, B_IDS, B_BOX, B_SY, B_SEQ);
            if (n >= 0 && bn > 0) {
                bN = 0;
                batchesFlushed++;
                batchEntries += bn;
            }
            if (n <= -2) {
                // capacity hint: the batch was consumed — regrow and retry
                // WITHOUT re-applying (same contract as the legacy regrow).
                if (growOutBuffer(-n)) {
                    // REBIND: dbb still points at the OLD registered buffer;
                    // the retry writes into the NEW one and the read loop
                    // below would IOOBE (and disarm the module) on the first
                    // regrow that actually grows — latent since doc 21.
                    dbb = IDS_BUF;
                    n = queryNative2(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                            0, B_IDS, B_BOX, B_SY, B_SEQ);
                }
            }
            if (n == -1) {
                mNotReady++;
                fastRefuse = "notready";
                return null; // index not ready / buffer vanished — mirror
            }
            if (n < 0) {
                mRegrowFail++;
                fastRefuse = "regrow";
                return null; // regrow failed — mirror
            }
            if (n > 0) {
                // paranoia guard: the read window must fit BOTH the Java
                // mirror array and the registered direct buffer — a mismatch
                // routes mirror (never IOOBEs, never disarms)
                if (n > ids.length) {
                    if (n > (1 << 20)) {
                        mRegrowFail++;
                        fastRefuse = "regrow2";
                        return null;
                    }
                    ids = new long[n];
                    QBUF.set(ids);
                }
                if ((long) n << 3 > dbb.capacity()) {
                    mRegrowFail++;
                    fastRefuse = "regrow3";
                    return null;
                }
                for (int k = 0; k < n; k++) {
                    ids[k] = dbb.getLong(k << 3);
                }
            }
        } else {
            // FLUSH-MERGED QUERY (iteration 3): the pending probe batch is
            // applied by the SAME native call that runs the query — one JNI
            // crossing and one index-lock acquisition per query (2b paid two
            // each: a flush before every query was ~1.7M flushes for ~1.9M
            // queries). batchN>0 only on the first attempt; retries must NOT
            // re-apply.
            n = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                    ids, bn, B_IDS, B_BOX, B_SY, B_SEQ);
            if (n >= 0 && bn > 0) {
                bN = 0;
                batchesFlushed++;
                batchEntries += bn;
            }
            for (int tries = 0; n >= 0 && n > ids.length && tries < 3; tries++) {
                ids = new long[n];
                QBUF.set(ids);
                n = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                        ids, 0, B_IDS, B_BOX, B_SY, B_SEQ);
            }
            if (n < 0) {
                mNotReady++;
                fastRefuse = "notready2";
                return null; // index not ready — mirror
            }
            if (n > ids.length) {
                mRegrowFail++;
                fastRefuse = "regrow4";
                return null; // regrow failed 3x — mirror
            }
        }
        if (prof) queryNanos += System.nanoTime() - t1;

        ArrayList<Entity> out = useScratch
                ? FQ_SCRATCH
                : new ArrayList<>(Math.min(n, 64));
        if (useScratch) { out.clear(); }
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
            if (MSQ_COLLECT && msqN < MSQ_IDS.length) {
                // the RAW candidate id — pre-window, pre-home, pre-box:
                // the MSQ diff names WHICH stage dropped a merge partner
                MSQ_IDS[msqN++] = cid;
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
            // L37d EXACT AABB RE-CHECK. The Rust index is a MARGIN SUPERSET:
            // EntityIndex.query expands the scan bounds by
            // max_half_span + MARGIN (the big-entity safety net), so a
            // candidate can sit up to ~0.7 blocks OUTSIDE the query box
            // (the parity ORDER-CHECK caught merge scans returning items
            // 0.64 blocks away — vanilla's section scan excludes them; the
            // extra candidates merged items beyond the merge radius and
            // pushed zombies outside the push box — THE item and mob
            // divergence in one). The vanilla test is
            // entity.getBoundingBox().intersects(box) on the LIVE box.
            AABB eb = e.getBoundingBox();
            // L44: an AFTER-scanner resync entry is seen at its PRE-body box
            // (the vanilla scan ran before that entry's body).
            if (isViewAfter(cid)) {
                eb = viewBox(cid, eb);
            }
            if (!(eb.minX < box.maxX && eb.maxX > box.minX
                    && eb.minY < box.maxY && eb.maxY > box.minY
                    && eb.minZ < box.maxZ && eb.maxZ > box.minZ)) {
                marginExtra++;
                // L41d: sample the first rejects' index-box vs LIVE-box —
                // the Rust scan already filters all 3 axes against the
                // INDEX box exactly, so an 84% rejection rate means the
                // index box disagrees with the live box. The sample names
                // which side is stale (the doc-41 exact-f64-query move).
                if (MSQ_COLLECT && msqRejN < 32) {
                    msqRej[msqRejN * 6] = eb.minX;
                    msqRej[msqRejN * 6 + 1] = eb.maxX;
                    msqRej[msqRejN * 6 + 2] = eb.minY;
                    msqRej[msqRejN * 6 + 3] = eb.maxY;
                    msqRej[msqRejN * 6 + 4] = eb.minZ;
                    msqRej[msqRejN * 6 + 5] = eb.maxZ;
                    msqRejIds[msqRejN] = cid;
                    msqRejN++;
                }
                continue;
            }
            // NOTE: the vanilla predicate still applies, as it must.
            if (predicate.test(e)) {
                out.add(e);
            }
        }
        if (prof) resolveNanos += System.nanoTime() - t2;
        // "resolved" for the parity assert means "the id mapped to a LIVE
        // entity" — the source self-hit (srcSkip) is exactly that (the
        // vanilla exclusion skips the PUSH, not the resolution). The L12
        // era shifted the e2e query mix toward merge scans (the zombies'
        // own pushes route the kernel now), which made self-hits dominate
        // the counters and the old resolved-only ratio read as a
        // resolution failure.
        candidatesResolved += resolved + srcSkip;
        staleCand += stale;
        // Desync means the index disagrees with the WORLD about LIVING
        // entities. All-stale candidates are the normal batching window, not
        // a desync — treating them as one falsely mirrored (and could even
        // suspend indexing) during merge storms.
        if (resolved == 0 && stale == 0 && srcSkip == 0 && n > 0) {
            mDesync++;
            return null; // desync guard
        }
        // L37: the index scan order is NOT the vanilla EntityLookup order —
        // the merge partner choice, the maxEntityCollisions cap and every
        // other order-sensitive consumer need the exact (chunkZ, chunkX,
        // sectionY, insertionSeq) sequence.
        ordSortVanilla(out);
        if (ORDER_CHECK) {
            orderChecks++;
            List<Entity> van = mirror(level, source, box, predicate);
            boolean same = van.size() == out.size();
            if (same) {
                for (int i = 0; i < out.size(); i++) {
                    if (van.get(i) != out.get(i)) {
                        same = false;
                        break;
                    }
                }
            }
            if (!same) {
                orderMismatches++;
                if (orderMismatches <= 12) {
                    StringBuilder sb = new StringBuilder(420);
                    sb.append("ORDER-MISMATCH n_van=").append(van.size())
                            .append(" n_fast=").append(out.size())
                            .append(" box=(")
                            .append(box.minX).append(',').append(box.minY).append(',').append(box.minZ)
                            .append(")-(")
                            .append(box.maxX).append(',').append(box.maxY).append(',').append(box.maxZ)
                            .append(')')
                            .append(" src=").append(srcId);
                    java.util.HashSet<Entity> vanSet = new java.util.HashSet<>(van);
                    int shown = 0;
                    for (Entity fe : out) {
                        if (!vanSet.contains(fe) && shown < 4) {
                            AABB fb = fe.getBoundingBox();
                            sb.append(" FASTONLY id=").append(fe.getId())
                                    .append(" pos=(").append(fe.getX()).append(',')
                                    .append(fe.getY()).append(',').append(fe.getZ())
                                    .append(") sec=(").append(fe.moonrise$getSectionZ())
                                    .append(',').append(fe.moonrise$getSectionX())
                                    .append(',').append(fe.moonrise$getSectionY())
                                    .append(") box6=(").append(fb.minX).append(',')
                                    .append(fb.minY).append(',').append(fb.minZ)
                                    .append(")-(").append(fb.maxX).append(',')
                                    .append(fb.maxY).append(',').append(fb.maxZ).append(')');
                            shown++;
                        }
                    }
                    java.util.HashSet<Entity> fastSet = new java.util.HashSet<>(out);
                    shown = 0;
                    for (Entity ve : van) {
                        if (!fastSet.contains(ve) && shown < 4) {
                            sb.append(" VANONLY id=").append(ve.getId());
                            shown++;
                        }
                    }
                    System.err.println("[c-entities] " + sb);
                }
            }
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
    // L32: widened 4 -> 16; L41b: 16 -> 36 — the staging rectangle is up to
    // 6x6 cells (raw span <= 3 + the 1-cell single-homing expansion per
    // axis); the scan below is already rectangle-general.
    private static final long[][] MQ_ROWS = new long[36][];
    private static final int[] MQ_CELL = new int[36];
    // L4c windowed rows: the UNWRAPPED cell x-origin of each covered row
    // (world blocks) — the reference frame for the packed candidate windows.
    private static final long[] MQ_CELL_X0 = new long[36];
    private static final long[] MQ_CELL_Z0 = new long[36];
    // Pending row-neutral ids homed in a covered cell: their packed windows
    // are stale until the next publish, so the window test must not skip them.
    // Sized to PENDING_CAP: every pending row-neutral id of a covered cell
    // MUST be recorded, or its stale window could false-skip it.
    private static final int[] WIN_IDS = new int[64];
    private static int winN = 0;

    private static List<Entity> rowQuery(
            net.minecraft.server.level.ServerLevel sl, int wid,
            Entity source, AABB box, Predicate<? super Entity> predicate,
            boolean useScratch) {
        // L41b CELL EXPANSION: an entity single-homes at its CENTER cell,
        // while vanilla's moonrise EntityLookup registers it in EVERY section
        // its BOX overlaps — a neighbor whose center sits just past a cell
        // boundary but whose box overlaps the query box is INVISIBLE to a
        // raw-cell row walk, while vanilla finds it. The native path expands
        // by max_half_span + MARGIN for exactly this single-homing safety;
        // the row path must too: a 1-cell expansion covers every center
        // within half-span <= 16 (bigger boxes are oversize-listed and never
        // homed here). run 40889274763: the items t=159 survivor flip
        // (971/1873) is a merge scan of self=971 returning n=0 where
        // vanilla found the cross-boundary 1873; mixed t=152 (469/964) —
        // the same shape.
        int cx0 = cellFloor(box.minX) - 1;
        int cx1 = cellFloor(box.maxX) + 1;
        int cz0 = cellFloor(box.minZ) - 1;
        int cz1 = cellFloor(box.maxZ) + 1;
        // L32: the window is 6x6 (span <= 5 after the expansion). Wider
        // boxes (the really wide targeting scans) still refuse into the
        // native/mirror paths.
        if (cx1 - cx0 > 5 || cz1 - cz0 > 5) { rowMulti++; fastRefuse = "multi"; return null; }
        // L35: narrow shapes (collision, merge scans) get the raised total
        // cap — the packed x-window below does the mass skip for them.
        double wxs = box.maxX - box.minX;
        double wzs = box.maxZ - box.minZ;
        boolean narrow = wxs <= 8.0 && wzs <= 8.0;
        int totalCap = narrow ? ROW_TOTAL_CAP_NARROW : ROW_TOTAL_CAP;
        int nr = 0;
        int total = 0;
        // L32 freshness proof for the null-skip below: the publisher only
        // republishes DIRTY cells (in place, same slot), so a null slot is a
        // cell never occupied in the current index epoch — an EMPTY cell.
        // An emptied cell is republished with count 0 (the move-out marks
        // both homes dirty), never cleared. A slot collision (cells beyond
        // the ±1024 direct range) cannot fake a null — and cannot fake a
        // hit either: the row key embeds the full ±2M-cell coords and the
        // key mismatch refuses. The one residual doubt — the index not yet
        // built for this world — is covered by requiring at least one
        // covered row to carry the current PUBLISH_SEQ before any null is
        // trusted: a live publish stamps every dirty cell it writes, so a
        // zero-fresh window means the index says nothing about it (early
        // boot, regrowth, foreign world) and the query refuses.
        boolean freshSeen = false;
        for (int cx = cx0; cx <= cx1; cx++) {
            for (int cz = cz0; cz <= cz1; cz++) {
                int packed = ((wid & 0x7) << 20) | (cx & 0x3FF) | ((cz & 0x3FF) << 10);
                long[] row = ROWS[packed];
                if (row == null) {
                    // L32: never-occupied cell — skipped, not refused (see
                    // the freshness proof above; the refusal decision needs
                    // the full window scanned first).
                    rowNoRow++;
                    continue;
                }
                long key = ((long) (wid & 0xFFFF) << 44)
                        | (((long) ((cx + (1 << 20)) & 0x3FFFFF)) << 22)
                        | (long) ((cz + (1 << 20)) & 0x3FFFFF);
                if (row[0] != key) {
                    rowKey++;
                    fastRefuse = "key";
                    if (dbgMisses < 3) {
                        dbgMisses++;
                        dbgExpKey = key;
                        dbgGotKey = row[0];
                        dbgExpSlot = packed;
                    }
                    return null;
                }
                long hdr = row[1];
                if ((hdr >>> 11) != PUBLISH_SEQ) { rowStale++; fastRefuse = "stale"; return null; }
                freshSeen = true;
                int n = (int) (hdr & 0x7FF);
                if (n > ROW_CAP) { rowBig++; fastRefuse = "bigdense"; return null; } // dense-kill sentinel
                MQ_ROWS[nr] = row;
                MQ_CELL[nr] = (cx & 0x3FF) | ((cz & 0x3FF) << 10);
                MQ_CELL_X0[nr] = (long) cx << 4;
                MQ_CELL_Z0[nr] = (long) cz << 4;
                nr++;
                total += n;
            }
        }
        if (!freshSeen) {
            // L32: every covered cell is either null or stale-fresh — the
            // index proves nothing about this window; the native/mirror
            // paths decide (the pre-L32 behavior for any non-empty window).
            fastRefuse = "nofresh";
            return null;
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
            if (bN > 64) { rowPending++; fastRefuse = "pending64"; return null; } // bound the scan
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
                        fastRefuse = "pending";
                        return null;
                    }
                }
            }
        } else {
            winN = 0;
        }
        if (total == 0) {
            // L16: the shared immutable empty list. At ~32k queries/tick the
            // per-query ArrayList(0) churned ~25MB/s of TLAB garbage that
            // shows up as allocator + GC pressure (Arrays.copyOf shows in the
            // leg). The query CONTRACT is read-only iteration; every in-tree
            // caller (push resolve, merge scan, goal targeting) only reads.
            // A mutating caller would throw UnsupportedOperationException and
            // flip the kill switch loudly — never silently corrupt.
            rowEmpty++;
            return java.util.Collections.emptyList();
        }
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
        if (total > totalCap) { rowBig++; fastRefuse = "bigcap"; return null; }
        ArrayList<Entity> out = useScratch
                ? FQ_SCRATCH
                : new ArrayList<>(Math.min(total, totalCap));
        out.clear();
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
            int rowSkipN = 0;
            if (MSQ_COLLECT) { msqSkipN = 0; }
            if (MSQ_COLLECT && msqRowsN < 12) {
                // L41e: the staged rows' (cx, cz, n) + the query interval —
                // the MSQ 971 case showed path=row rawN=0 win=163 (every
                // candidate window-rejected) while the neighbor scan of the
                // SAME cell found the merge partner: the staged-cell/window
                // dump pins the exact disagreement.
                msqRows[msqRowsN * 4] = (cellX0 >> 4);
                msqRows[msqRowsN * 4 + 1] = (MQ_CELL_Z0[r] >> 4);
                msqRows[msqRowsN * 4 + 2] = n;
                msqRows[msqRowsN * 4 + 3] = 0;
                msqRowsN++;
            }
            double x0 = (double) cellX0;          // exact: |cx| << 49
            double z0 = (double) MQ_CELL_Z0[r];
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
                // L44: an AFTER-scanner resync entry's packed window rides its
                // POST-move box — the vanilla scan view is the PRE-move box,
                // so the window must not skip it.
                if (winTrusted && isViewAfter(cid)) winTrusted = false;
                if (winTrusted
                        && ((((int) (p >>> 48)) & 0xFFFF) < qLo
                            || (((int) (p >>> 32)) & 0xFFFF) > qHi)) {
                    rowWinSkip++;
                    // L44b: name the skipped candidates PER ROW (the packed
                    // interval vs the row's qLo/qHi pins the staleness)
                    if (MSQ_COLLECT && msqSkipN < 16) {
                        msqSkipId[msqSkipN] = cid;
                        msqSkipLo[msqSkipN] = ((int) (p >>> 32)) & 0xFFFF;
                        msqSkipHi[msqSkipN] = ((int) (p >>> 48)) & 0xFFFF;
                        msqSkipRow[msqSkipN] = rowSkipN++;
                        msqSkipN++;
                    }
                    continue;
                }
                Entity e = ARENA[cid & AMASK];
                if (e == null || e.getId() != cid) {
                    e = sl.getEntity(cid); // LevelEntityGetter.byId — id-keyed
                    if (e == null) { stale++; continue; }
                }
                AABB eb = e.getBoundingBox();
                // L44: an AFTER-scanner resync entry is seen at its PRE-body
                // box (the vanilla scan ran before that entry's body).
                boolean viewAfter = isViewAfter(cid);
                if (viewAfter) {
                    eb = viewBox(cid, eb);
                }
                // home re-check (exactness): the entity's CURRENT home must
                // still be the cell whose row we are reading; a pending
                // cross-cell move drops out here exactly like vanilla's
                // section semantics. L15: the row's exact origins are known,
                // so the floor(center/16)==cell test becomes interval tests —
                // no per-candidate division (the cellOfBox form cost 1.26%
                // caller CPU in the 26.3 mobs-25k-mod profile). Equivalence:
                // floor(center)==c as doubles <=> center in [c*16, c*16+16),
                // and (min+max)*0.5 is exact (division by 2), so comparing
                // center against the [x0, x0+16) interval is the SAME test.
                // L44: view entries are dropped by the POST-home row but
                // vanilla saw them registered at their PRE-move sections —
                // the (PRE-)box test below is authoritative for them.
                if (!viewAfter) {
                    double ecx = (eb.minX + eb.maxX) * 0.5;
                    if (ecx < x0 || ecx >= x0 + 16.0) {
                        homeMiss++;
                        if (MSQ_COLLECT && msqHmN < 32) { msqHmId[msqHmN++] = cid; }
                        continue;
                    }
                    double ecz = (eb.minZ + eb.maxZ) * 0.5;
                    if (ecz < z0 || ecz >= z0 + 16.0) {
                        homeMiss++;
                        if (MSQ_COLLECT && msqHmN < 32) { msqHmId[msqHmN++] = cid; }
                        continue;
                    }
                }
                if (eb.intersects(box)) {
                    if (MSQ_COLLECT && msqN < MSQ_IDS.length) {
                        MSQ_IDS[msqN++] = cid;
                    }
                    if (predicate == null || predicate.test(e)) {
                        out.add(e);
                    }
                } else if (MSQ_COLLECT && msqRejN < 32) {
                    // row-path reject sample (the native resolve had one; the
                    // 971 scan's 13 window-passed candidates were box-rejected
                    // invisibly — this names them).
                    msqRej[msqRejN * 6] = eb.minX;
                    msqRej[msqRejN * 6 + 1] = eb.maxX;
                    msqRej[msqRejN * 6 + 2] = eb.minY;
                    msqRej[msqRejN * 6 + 3] = eb.maxY;
                    msqRej[msqRejN * 6 + 4] = eb.minZ;
                    msqRej[msqRejN * 6 + 5] = eb.maxZ;
                    msqRejIds[msqRejN] = cid;
                    msqRejN++;
                }
            }
            // L44b: per-row skip dump — the packed interval vs this row's
            // window, with the row origin, pins a stale row entry exactly.
            if (MSQ_COLLECT && msqSkipN > 0) {
                StringBuilder sb3 = new StringBuilder(240);
                for (int k = 0; k < msqSkipN; k++) {
                    sb3.append(k == 0 ? "" : " ").append(msqSkipId[k])
                            .append(':').append(msqSkipLo[k])
                            .append('-').append(msqSkipHi[k]);
                }
                System.out.println("[ce-baseline] MSSR t=" + CUR_TICK
                        + " row=(" + (cellX0 >> 4) + ',' + (MQ_CELL_Z0[r] >> 4) + ')'
                        + " win=[" + qLo + ',' + qHi + "] ox=" + cellX0
                        + " skip=" + rowSkipN + " idLoHi=" + sb3);
                msqSkipN = 0;
            }
        }
        rowStaleCand += stale;
        rowHomeMiss += homeMiss;
        // (per-row MSS lines print inside the row loop — see MSSR above)
        rowFast++;
        if (narrow) rowNarrow++;
        // L37: the row walk order (cells, rows, packed windows) is not the
        // vanilla EntityLookup order — same sort as the native path.
        ordSortVanilla(out);
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
    // gateCell → worldId. History: a ConcurrentHashMap.get cost ~40-60ns; the
    // 4-slot scan + 1-slot LRU pair fixed the common case but the JFR profile
    // of the 26.3 mobs-25k-mod leg still showed 2.05% self time at the
    // putIfAbsent line: with MORE live worlds than the 4 slots (overworld +
    // nether + end exist by default) and alternating callers, the 1-slot LRU
    // missed on every alternation and ALLOCATED a fresh Object[2] per hit,
    // while worlds with id >= 4 fell through to the map on EVERY call.
    //
    // L15: one flat identity scan over 16 slots, zero allocation on any hit,
    // map only on the very first sight of a level. Slots are published once
    // (world ids are never reassigned) and never rebound, so a reader racing
    // a publisher sees a null slot and takes the map path — never a wrong id.
    // Plain arrays + benign races: reference writes are atomic, both racers
    // write the same immutable pair, and the slot order is stable because a
    // slot is claimed only when null (winner writes its own id there).
    private static final net.minecraft.world.level.Level[] WID_SLOTS = new net.minecraft.world.level.Level[16];

    // L27: the single-slot memo. worldId is called >1M times/tick (query,
    // probe and push bridges, one to three times per entity per tick); the
    // L16 unroll costs 1-3 static-array loads + compares. One immutable
    // (level, id) pair published atomically through a volatile field is ONE
    // load + one identity compare on the hot path — and no torn-pair hazard:
    // the L19 idea of two plain statics (last level, last id) can interleave
    // a server-thread write with a worldgen-worker write into a mismatched
    // pair (level of one world, id of another — a cross-world index leak).
    // A single volatile reference publish is atomic; writers race benignly
    // (any published pair is self-consistent, and the fall-through path
    // still resolves and publishes the correct one). The dead L19 widFast
    // (zero callers — the hot paths all call worldId directly) is removed
    // with its torn-pair pair.
    private record WidMemo(net.minecraft.world.level.Level level, int id) {
    }

    private static volatile WidMemo WID_LAST;

    public static int worldId(net.minecraft.world.level.Level level) {
        WidMemo m = WID_LAST;
        if (m != null && m.level == level) return m.id();
        int w = resolveWorldId(level);
        WID_LAST = new WidMemo(level, w);
        return w;
    }

    private static int resolveWorldId(net.minecraft.world.level.Level level) {
        // L16: the benchmarks keep exactly THREE live query worlds
        // (overworld, nether, end) in slots 0-2 — the unrolled identity
        // probes below hit on ~all calls with 1-3 compares and no loop
        // bookkeeping. The general 16-slot scan stays for any shape beyond
        // that (worlds >= 3, plugin worlds), then the map.
        if (WID_SLOTS[0] == level) return 0;
        if (WID_SLOTS[1] == level) return 1;
        if (WID_SLOTS[2] == level) return 2;
        for (int w = 3; w < 16; w++) {
            if (WID_SLOTS[w] == level) {
                return w;
            }
            if (WID_SLOTS[w] == null) break; // no slots beyond the first null
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
        if (w < 16) {
            if (WID_SLOTS[w] == null) WID_SLOTS[w] = level;
            else if (WID_SLOTS[w] != level) {
                // slot collision (should be impossible: slot w is only ever
                // claimed by world w): keep the table sound by leaving it —
                // this world stays on the map path.
                mWorldSlotClash++;
                return w;
            }
        }
        return w;
    }

    /** Kill-switch arm/disarm from the native side. */
    public static void setActive(boolean on) {
        ACTIVE = on;
        if (on) {
            // doc 21: register the direct out-buffer for queryNative2. The
            // natives are already registered in the arm sequence; any failure
            // leaves IDS_BUF null and fastQuery stays on the legacy path.
            if (IDS_BUF == null) {
                try {
                    java.nio.ByteBuffer bb = java.nio.ByteBuffer.allocateDirect(1024 << 3);
                    // RAW native stores are little-endian; a direct buffer
                    // defaults to BIG_ENDIAN and would byte-reverse every id
                    // (the (int) cast then lands on 0 -> 100% arena miss).
                    bb.order(java.nio.ByteOrder.LITTLE_ENDIAN);
                    int got = nativeSetOutBuffer(bb);
                    if (got > 0) {
                        IDS_BUF = bb;
                        IDS_BUF_CAP = got;
                    }
                } catch (Throwable t) {
                    lastErrorText = "outbuf: " + t;
                }
            }
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
            // L37: entities added BEFORE the arm (world load during boot)
            // never passed probeAdd/probeMoveSection — stamp them in the
            // EXACT vanilla scan order (a full-world EntityLookup walk IS
            // the vanilla order). A failure leaves them unstamped (seq 0):
            // they tie and sort in index order — the fast path still serves
            // the exact SET, only the order of pre-arm entities degrades.
            try {
                ordBackfillWorlds();
            } catch (Throwable t) {
                lastErrorText = "ordBackfill: " + t;
            }
        }
        if (!on) {
            cplugReady = false;
        }
    }

    /** L15 arm switches — called by the native verifier after the
     *  corresponding replace_body weave is PROVEN live (counters move).
     *  Each gate arms independently; a refusal keeps vanilla forever. */
    public static void setAiFloorOn(boolean on) {
        AI_ON = on;
    }

    public static void setFrozenGateOn(boolean on) {
        FROZEN_ON = on;
    }

    public static void setConvGateOn(boolean on) {
        CONV_ON = on;
    }

    public static void setTrackerTickGateOn(boolean on) {
        TT_ON = on;
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
            long[] out, int batchN, long[] batchIds, double[] batchBoxes,
            int[] batchSy, int[] batchSeq);
    public static native void nativeFlush(
            int n, long[] ids, double[] boxes,
            int[] pf, double[] pxz, double[] dv, int[] vid,
            int[] sy, int[] seq,
            int dbn, long[] dbIds, double[] dbDv);
    /** L12 push kernel: compute one `LivingEntity.pushEntities()` natively.
     *  Returns a packed long: bit0 = ok (vanilla body suppressed), bit1 =
     *  cramming audit needed, bits 2..21 = non-passenger hit count,
     *  bits 22..53 = numCollisions after the tick. The post-tick delta is
     *  written into PUSH_SDX/SDY/SDZ (server-thread confined statics). */
    public static native long nativePush(
            int wid, int selfId, int selfVehicle, int selfPlayer,
            int onlyPlayers, int maxEC, int cramming, int curTick);
    public static native void nativeBoxDirect(
            long id, int world, double x0, double y0, double z0, double x1, double y1, double z1);
    public static native void nativeRemoveDirect(long id);
    public static native void nativeReset();

    // ------------------------------------------------------------------
    // doc 21 — DIRECT OUT BUFFER: the query result ids land in a pinned
    // direct ByteBuffer the native side writes with raw pointer stores —
    // ZERO per-query JNI array ops (no get_array_length, no
    // set_long_array_region, no GC write barriers). The buffer is
    // registered once at arm (and regrown only when a query outgrows it);
    // a NewGlobalRef keeps it alive on the native side. Java reads ids via
    // the intrinsified ByteBuffer.getLong (a single Unsafe load each).
    //
    // Contract of queryNative2 (no out-array — results go to the buffer):
    //   n >= 0  — count written into the buffer
    //   n == -1 — index not ready / buffer vanished (caller: legacy path)
    //   n <= -2 — required capacity is -n (caller regrows and retries
    //             WITHOUT re-applying the batch — it was consumed)
    // ------------------------------------------------------------------
    public static native int queryNative2(
            int world, double x0, double y0, double z0, double x1, double y1, double z1,
            int batchN, long[] batchIds, double[] batchBoxes,
            int[] batchSy, int[] batchSeq);
    /** Registers a direct buffer as the query out-buffer; returns the
     *  capacity in longs, or 0 on any failure (legacy path stays armed). */
    public static native int nativeSetOutBuffer(java.nio.ByteBuffer buf);

    /** Direct out-buffer (server-thread confined; written by the arm thread
     *  before ACTIVE=true publishes — the volatile pair gives visibility). */
    private static volatile java.nio.ByteBuffer IDS_BUF;
    private static volatile int IDS_BUF_CAP;

    private static boolean growOutBuffer(int needLongs) {
        if (needLongs <= 0 || needLongs > (1 << 20)) return false;
        int cap = Math.max(needLongs, Math.max(IDS_BUF_CAP * 2, 1024));
        try {
            java.nio.ByteBuffer nb = java.nio.ByteBuffer.allocateDirect(cap << 3);
            // see setActive: the native side writes little-endian i64s
            nb.order(java.nio.ByteOrder.LITTLE_ENDIAN);
            int got = nativeSetOutBuffer(nb);
            if (got > 0) {
                IDS_BUF = nb;
                IDS_BUF_CAP = got;
                return true;
            }
        } catch (Throwable t) {
            lastErrorText = "growOut: " + t;
        }
        return false;
    }

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

    // MS diagnostics: the merge scan streams its candidate list when the
    // current tick is inside the PH_DUMP window (one range — enough to aim
    // the parity diff). CUR_TICK bumps once per itemTickFlush (the scans
    // run only inside the flush, so it is exact for them).
    private static final int[] MS_WINDOW = parseMsWindow(System.getenv("PH_DUMP"));
    public static volatile int CUR_TICK = 0;

    private static int[] parseMsWindow(String s) {
        if (s == null || s.isBlank()) return new int[0];
        try {
            int dash = s.indexOf('-');
            int lo, hi;
            if (dash > 0) {
                lo = Integer.parseInt(s.substring(0, dash).trim());
                hi = Integer.parseInt(s.substring(dash + 1).split(",")[0].trim());
            } else {
                lo = hi = Integer.parseInt(s.trim().split(",")[0]);
            }
            // PH_DUMP is in PLUGIN-tick space; CUR_TICK counts flushes across
            // ALL loaded levels (3 at parity: overworld+nether+end — run
            // 40789269145: ct=423 at t=141) — scale and pad.
            lo = lo * 3 - 15;
            hi = hi * 3 + 15;
            return new int[]{lo, hi};
        } catch (NumberFormatException quiet) {
            return new int[0];
        }
    }

    private static boolean phDumpTickNow() {
        return MS_WINDOW.length == 2
                && CUR_TICK >= MS_WINDOW[0] && CUR_TICK <= MS_WINDOW[1];
    }
    // L15b: PLAIN (per item-tick, 100k/tick) — see the mergeChecks note.
    public static long itemChecks = 0, itemNative = 0, itemInactive = 0,
            itemResync = 0, itemVanillaRouted = 0, itemMerge = 0, itemDespawn = 0,
            itemLand = 0, itemVoid = 0, itemVanillaNext = 0, itemAbort = 0, itemFlushes = 0;

    // ---- batch (server-thread confined; one world per tick segment) ----
    private static final int IB_CAP = 1 << 17;
    private static final long[] IB_IDS = new long[IB_CAP];
    private static final int[] IB_FLAGS = new int[IB_CAP];
    // L21: the entity ref captured AT COLLECT — the flush applies through
    // this (one array read) instead of arenaRef(id) (an ARENA read + the
    // megamorphic cand.getId() staleness call: measured 15.6% of the
    // items-100k-mod leg). The batch is per-tick state, so no id-match
    // staleness check is needed: a mid-tick removal is caught by
    // isRemoved() (a plain field read).
    private static final Entity[] IB_REF = new Entity[IB_CAP];
    // L8.2 COMPACT BATCH: only collector-data entries (resync /
    // force-vanilla — the rare population) carry the 14-double state stride;
    // they are side-packed contiguously into IB_CD. The 1-based slot number
    // rides in IB_FLAGS bits 6+ (0 = dispatch-owned, no per-tick fields).
    // The old layout copied n*14 doubles (~7MB/tick of 99%-dead zeros at
    // 60k items) through JNI every flush.
    private static double[] IB_CD = new double[1024 * 14];
    private static int ibCDN = 0;
    private static final int IF_CDSHIFT = 6;
    private static final int IF_CDMASK = 0x3F; // the IF_* bit namespace
    private static final int[] IB_I = new int[IB_CAP * 4];        // tickCount, age, pickupDelay, despawnRate
    private static int ibN = 0;

    // flag bits (must match itemcore.rs)
    public static final int IF_INACTIVE = 1;        // entry came from inactiveTick
    public static final int IF_NATIVE = 2;          // suppress vanilla: Rust computes
    public static final int IF_RESYNC = 4;          // vanilla body ran (or forced): ingest only
    public static final int IF_FRIC_OFF = 8;        // frictionState == FALSE -> f = 1.0
    public static final int IF_FORCE_VANILLA = 16;  // fresh-flag gate: vanilla this tick
    public static final int IF_ON_GROUND = 32;      // e.onGround() at collect time

    // L12 push flag bits (must match pushcore.rs PF_*)
    public static final int PF_PUSHABLE = 1;
    public static final int PF_SPECTATOR = 2;
    public static final int PF_NOPHYSICS = 4;
    public static final int PF_PASSENGER = 8;
    public static final int PF_VEHICLE = 16;
    public static final int PF_PLAYER = 32;
    public static final int PF_NEEDSSYNC = 64;
    /** L12 kernel outputs (server-thread confined): the post-tick delta and
     *  the packed verdict of the last nativePush call. */
    public static double PUSH_SDX = 0.0, PUSH_SDY = 0.0, PUSH_SDZ = 0.0;
    /** L42 pushed-pair outputs (server-thread confined): the kernel emits
     *  (id, ddx, ddz) per pushed candidate instead of mutating the mirror —
     *  the apply loop writes the candidate's delta THIS TICK (vanilla's
     *  doPush lands both sides' forces inside the pusher's body), and the
     *  write's own probe event re-syncs the mirror. Sized for the
     *  maxEntityCollisions cap with headroom; the kernel caps the emission. */
    public static final int PUSH_PAIR_CAP = 256;
    public static final long[] PUSH_PAIR_IDS = new long[PUSH_PAIR_CAP];
    public static final double[] PUSH_PAIR_D = new double[PUSH_PAIR_CAP * 2];
    public static int PUSH_PAIR_N = 0;
    // L15b: PLAIN (per push, 25k/tick on mobs-25k) — see the mergeChecks note.
    public static long pushChecks = 0, pushNative = 0, pushMirror = 0,
            pushCramming = 0, pushSelfDelta = 0, pushPairs = 0, pushPairMiss = 0;
    /** L12 kill switch: -Dcentities.pushNative=false keeps every push vanilla. */
    public static final boolean PUSH_ON =
            !"false".equals(System.getProperty("centities.pushNative", "true"));
    /** Lazy private-access setter for the PROTECTED Entity.numCollisions field
     *  (the vanilla pushEntities counter — the only reader/writer outside is
     *  that body, javap-verified, so the kernel mirror is authoritative and
     *  this setter only re-syncs the Java field). */
    private static volatile java.lang.invoke.MethodHandle MH_NCOL;
    /** The PROTECTED LivingEntity.random field + RandomSource.nextInt(int) —
     *  the cramming audit's RNG must be the entity's own source, consumed in
     *  exactly the vanilla conditions. */
    private static volatile java.lang.invoke.MethodHandle MH_RANDOM;
    private static volatile java.lang.invoke.MethodHandle MH_NEXT_INT;
    private static volatile boolean MH_NCOL_FAILED = false;
    /** L42: the Unsafe offset of the PROTECTED Entity.numCollisions field —
     *  the pushed candidates' per-pair increment (vanilla entityx
     *  .numCollisions++ in the pusher's body) must reach the Java field
     *  even when the candidate's own kernel run never writebacks it. */
    private static long O_NCOL = -1;

    private static synchronized boolean initNcolMh(Entity sample) {
        if (MH_NCOL_FAILED) return false;
        if (MH_NCOL != null && MH_RANDOM != null && MH_NEXT_INT != null) return true;
        // ATOMIC: build ALL handles into locals; publish only on complete
        // success (the first kernel round died on a half-published set —
        // MH_NCOL live, MH_RANDOM null — and the cramming branch NPE'd)
        try {
            java.lang.reflect.Field f = net.minecraft.world.entity.Entity.class
                    .getDeclaredField("numCollisions");
            f.setAccessible(true);
            java.lang.invoke.MethodHandle ncol =
                    java.lang.invoke.MethodHandles.lookup().unreflectSetter(f)
                            .asType(java.lang.invoke.MethodType.methodType(
                                    void.class, Entity.class, int.class));
            // 26.3 removed Entity.getRandom()/LivingEntity.getRandom() — the
            // RandomSource is the protected final Entity.random FIELD
            // (javap-verified: no getRandom on Entity/LivingEntity/Mob;
            // observed: NoSuchMethodException killed the whole ncolMh bind
            // and silently vanilla-routed the push kernel in every leg).
            java.lang.reflect.Field rf = net.minecraft.world.entity.Entity.class
                    .getDeclaredField("random");
            rf.setAccessible(true);
            java.lang.invoke.MethodHandle rnd =
                    java.lang.invoke.MethodHandles.lookup().unreflectGetter(rf)
                            .asType(java.lang.invoke.MethodType.methodType(
                                    net.minecraft.util.RandomSource.class,
                                    net.minecraft.world.entity.LivingEntity.class));
            java.lang.reflect.Method ni = net.minecraft.util.RandomSource.class
                    .getDeclaredMethod("nextInt", int.class);
            java.lang.invoke.MethodHandle next =
                    java.lang.invoke.MethodHandles.lookup().unreflect(ni)
                            .asType(java.lang.invoke.MethodType.methodType(
                                    int.class, net.minecraft.util.RandomSource.class, int.class));
            MH_NCOL = ncol;
            MH_RANDOM = rnd;
            MH_NEXT_INT = next;
            O_NCOL = U.objectFieldOffset(f);
            return true;
        } catch (Throwable t) {
            MH_NCOL = null;
            MH_RANDOM = null;
            MH_NEXT_INT = null;
            O_NCOL = -1;
            MH_NCOL_FAILED = true;
            lastError = System.nanoTime();
            lastErrorText = "ncolMh: " + t;
            return false;
        }
    }

    /**
     * L12 — PUSH DISPATCH TAKEOVER. Woven as an EarlyReturnGuard on
     * LivingEntity.pushEntities(): true = the vanilla body is suppressed,
     * the Rust kernel computed this entity's whole push tick (candidate
     * scan over the pushable-only index, exact f64 intersects, the Entity.push
     * force math in vanilla candidate order, numCollisions bookkeeping, the
     * cramming audit inputs). The Java tail below applies the self writeback
     * (delta + needsSync + numCollisions) and the cramming hurt with the
     * entity's own RandomSource, in the vanilla order.
     *
     * Any uncertainty returns false — the vanilla body runs. When the kernel
     * HAS run for this entity before (its numCollisions mirror is ahead of
     * the Java field), the decline path re-syncs the mirror into the field
     * so the vanilla body starts from the right counter.
     */
    /** Per-tick config cache: gamerules/paperConfig/listener checks are
     *  map lookups + array scans; 15k pushers/tick paid ~2-3ms for them.
     *  Refreshed by the first pusher of each tick (server-thread confined). */
    private static int PUSH_CFG_TICK = -1;
    private static int PUSH_CFG_MAXEC = 0;
    private static int PUSH_CFG_CRAMMING = 0;
    private static boolean PUSH_CFG_ONLYPLAYERS = false;
    private static boolean PUSH_CFG_PLAYERS = true;
    private static boolean PUSH_CFG_EVENTS = false;

    private static void refreshPushCfg(net.minecraft.server.level.ServerLevel sl) {
        int t = net.minecraft.server.MinecraftServer.currentTick;
        if (PUSH_CFG_TICK == t) return;
        PUSH_CFG_TICK = t;
        PUSH_CFG_MAXEC = sl.paperConfig().collisions.maxEntityCollisions;
        PUSH_CFG_CRAMMING = sl.getGameRules()
                .get(net.minecraft.world.level.gamerules.GameRules.MAX_ENTITY_CRAMMING);
        PUSH_CFG_ONLYPLAYERS = sl.paperConfig().collisions.onlyPlayersCollide;
        PUSH_CFG_PLAYERS = io.papermc.paper.configuration.GlobalConfiguration
                .get().collisions.enablePlayerCollisions;
        try {
            PUSH_CFG_EVENTS = io.papermc.paper.event.entity.EntityCollideWithEntityEvent
                    .getHandlerList().getRegisteredListeners().length != 0;
        } catch (Throwable t2) {
            PUSH_CFG_EVENTS = true; // uncertain -> vanilla
        }
    }

    public static boolean pushDispatch(Entity ent) {
        pushChecks++;
        if (!ACTIVE || !PUSH_ON || !cplugReady) return false;
        if (!(ent instanceof net.minecraft.world.entity.LivingEntity le)
                || ent instanceof net.minecraft.server.level.ServerPlayer) {
            return false;
        }
        try {
            net.minecraft.world.level.Level w = le.level();
            if (w.isClientSide() || !(w instanceof net.minecraft.server.level.ServerLevel sl)) {
                return false;
            }
            if (le.isRemoved() || le.noPhysics || le.isPassenger()) return false;
            // team collision rules: the predicate folds per-pair team logic;
            // any team on either side routes vanilla. (The isPushable slice
            // lives in the ARENA flags now — refreshed per box/add event —
            // the per-call recompute cost 15k pushers their chunk lookups.)
            if (le.getTeam() != null) return false;
            refreshPushCfg(sl);
            if (PUSH_CFG_EVENTS) return false;
            int wid = worldId(w);
            if (wid < 0) return false;
            boolean selfVehicle = le.isVehicle();
            // the kernel reads the mirror: drain the pending probe batches first
            flushIfPending();
            long res = nativePush(wid, le.getId(), selfVehicle ? 1 : 0,
                    PUSH_CFG_PLAYERS ? 1 : 0, PUSH_CFG_ONLYPLAYERS ? 1 : 0,
                    PUSH_CFG_MAXEC, PUSH_CFG_CRAMMING, PUSH_CFG_TICK);
            if ((res & 1L) == 0L) {
                // kernel declined (arena miss / index not ready): sync the
                // mirror's numCollisions into the Java field, then vanilla.
                // L41c: the mirror value is authoritative ONLY when the
                // kernel computed this entity's push THIS tick (the native
                // gates it on its ncol_tick) — a stale mirror value would
                // RESET the vanilla numCollisions (the vanilla body updates
                // the real field every tick; the mirror never sees those
                // writes) and remove the maxEntityCollisions cap from the
                // vanilla pushes that follow (the mobs 377 trace: the ON
                // leg's delta carried the un-capped push forces).
                pushMirror++;
                int ncolMirror = nativePushNcol(le.getId(), PUSH_CFG_TICK);
                if (ncolMirror >= 0 && MH_NCOL != null) {
                    try {
                        MH_NCOL.invokeExact((Entity) le, ncolMirror);
                    } catch (Throwable ignore) { /* vanilla counter stays */ }
                }
                return false;
            }
            pushNative++;
            int ncolNew = (int) ((res >>> 22) & 0xFFFF_FFFFL);
            int nonPass = (int) ((res >>> 2) & 0xF_FFFFL);
            // self writeback — the same calls the vanilla body would have
            // made, in the same order (setDeltaMovement allocates a fresh
            // Vec3 exactly like vanilla; needsSync marks the tracker)
            le.setDeltaMovement(new Vec3(PUSH_SDX, PUSH_SDY, PUSH_SDZ));
            le.needsSync = true;
            pushSelfDelta++;
            if (MH_NCOL == null && !initNcolMh(le)) {
                return false; // cannot maintain the counter — vanilla next tick
            }
            try {
                MH_NCOL.invokeExact((Entity) le, ncolNew);
            } catch (Throwable t) {
                lastError = System.nanoTime();
                lastErrorText = "pushNcol: " + t;
                ACTIVE = false;
                return false;
            }
            if ((res & 2L) != 0L) {
                // vanilla: list.size() > cramming-1 && random.nextInt(4) == 0
                // && nonPassengers > cramming-1 -> hurtServer(cramming, 6f).
                // The RNG handles are part of the kernel contract: without
                // them the tick cannot consume the entity's RandomSource in
                // the vanilla condition -> route vanilla entirely.
                if (MH_RANDOM == null || MH_NEXT_INT == null) {
                    return false;
                }
                pushCramming++;
                int roll = -1;
                try {
                    net.minecraft.util.RandomSource rnd = (net.minecraft.util.RandomSource) MH_RANDOM.invokeExact(le);
                    roll = (int) MH_NEXT_INT.invokeExact(rnd, 4);
                } catch (Throwable t) {
                    lastError = System.nanoTime();
                    lastErrorText = "pushRng: " + t;
                    ACTIVE = false;
                    return false;
                }
                if (roll == 0 && nonPass > PUSH_CFG_CRAMMING - 1) {
                    le.hurtServer(sl, sl.damageSources().cramming(), 6.0f);
                }
            }
            // L42: the pushed candidates' own writes — vanilla applies them
            // INSIDE the pusher's body (doPush -> entityx.push(this) mutates
            // BOTH deltas; the audit runs BEFORE the pair loop, hence this
            // placement). The old mirror-only deferral lost the force
            // whenever the candidate's own kernel run declined, and the
            // pusher's writeback then carried the stale mirror base — the
            // first-move PH divergence (doc 40 addendum 5). entityx
            // .numCollisions++ precedes doPush in vanilla; the push() call
            // is the EXACT vanilla 3-arg overload (no event, needsSync,
            // the probe captures the new value for the mirror).
            if (O_NCOL < 0) {
                return false; // cannot maintain the candidates' counters
            }
            int pn = PUSH_PAIR_N;
            for (int i = 0; i < pn; i++) {
                long pid = PUSH_PAIR_IDS[i];
                Entity pe = ARENA[(int) (pid & AMASK)];
                if (pe == null || pe.getId() != (int) (pid & 0xFFFF_FFFFL)) {
                    // unreachable once the JNI side writes the arrays IN
                    // PLACE: the pair ids come from the mirror the ARENA
                    // feeds, no Java code runs between n_push and this loop.
                    // (The first CI run missed 210/223 because the arrays
                    // were static-final and the JIT folded the ORIGINAL
                    // empty-array reference while n_push swapped the field
                    // to a fresh array each call — the compiled loop read
                    // all-zero ids. The region-write fix keeps the object.)
                    pushPairMiss++;
                    continue;
                }
                U.putInt(pe, O_NCOL, U.getInt(pe, O_NCOL) + 1);
                pe.push(PUSH_PAIR_D[i * 2], 0.0, PUSH_PAIR_D[i * 2 + 1]);
            }
            pushPairs += pn;
            // L47 PUSH FORENSICS: the mixed t=233 PTRACE proved an EXTRA
            // push on a resting armor stand (ON gained vx/vz at tc=90,
            // vanilla stayed zero; spread t=235 4142 and mobs t=275 1069
            // are the same one-tick family) — the kernel accepted a pair
            // vanilla's in-phase scan excludes. At PD ticks, dump the
            // pusher's identity + live box and the exact pair set with
            // forces; the OFF leg's PD/PTRACE supplies the vanilla outcome,
            // so the diff names the culprit pair and its geometry.
            if (phDumpTickNow()) {
                StringBuilder pb = new StringBuilder(96 + pn * 24);
                AABB sbx = le.getBoundingBox();
                pb.append("[ce-baseline] KPH t=").append(CUR_TICK)
                        .append(" self=").append(le.getId())
                        .append(" tc=").append(le.tickCount)
                        .append(" n=").append(pn)
                        .append(" box=[").append(sbx.minX).append(',').append(sbx.maxX)
                        .append(' ').append(sbx.minY).append(',').append(sbx.maxY)
                        .append(' ').append(sbx.minZ).append(',').append(sbx.maxZ)
                        .append(']');
                for (int i = 0; i < Math.min(pn, 16); i++) {
                    pb.append(' ').append(PUSH_PAIR_IDS[i] & 0xFFFF_FFFFL)
                            .append(':').append(PUSH_PAIR_D[i * 2])
                            .append(',').append(PUSH_PAIR_D[i * 2 + 1]);
                }
                System.out.println(pb);
            }
            PUSH_PAIR_N = 0;
            return true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "pushDispatch: " + t;
            t.printStackTrace();
            ACTIVE = false;
            return false;
        }
    }

    /** The numCollisions mirror value for `id`, or -1 (mirror miss). Read
     *  through a tiny native so the decline path can re-sync the field. */
    private static native int nativePushNcol(int id, int curTick);

    /** Drain any pending probe batches before a kernel call: the kernel
     *  reads the push mirror, so its view must be current. No-op when the
     *  batch is empty (the steady state — the DRAIN_CAP=64 policy keeps the
     *  batch small, and the kernel call is per-entity). */
    static void flushIfPending() {
        flush();
    }

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
    // L15b: PLAIN (per non-player tick) — see the mergeChecks note.
    public static long dispChecks = 0, dispNative = 0,
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
            if (e.level().isClientSide()) return false;
            int slot = e.getId() & MAMASK;
            if (MODE_VAL[slot] == 2) { // L21 REST: the dispatch-owned fixed point
                if (REST_ID[slot] != e.getId()) {
                    // slot turnover: entity ids are monotonic per run, so a
                    // mismatched stamp belongs to a dead predecessor
                    MODE_VAL[slot] = 0;
                } else {
                    // cheap gates (the same set the native path runs): any
                    // hit leaves rest — vanilla computes the tick exactly
                    // and the resync re-arms the mode (the L6 protocol)
                    if (e.isRemoved() || e.noPhysics
                            || e.portalProcess != null || e.isOnPortalCooldown()
                            || stackEmptyDispatch(e)
                            || !e.getPassengers().isEmpty()
                            || ARENA[e.getId() & AMASK] != e) {
                        MODE_VAL[slot] = 0;
                        restExits++;
                        dispPass++;
                        return false;
                    }
                    if (MODE_REV[slot] != WORLD_REV) {
                        dispGate++;
                        // the global revision moved: exit only when THIS
                        // item's environment was touched (a block change
                        // anywhere must not herd the whole rest set)
                        if (e.isInWater() || e.isInLava() || fireTicks(e) != 0
                                || despawnTimeOf(e) >= 0 || restSectionTouched(e)) {
                            MODE_VAL[slot] = 0;
                            restExits++;
                            dispPass++;
                            return false;
                        }
                        MODE_REV[slot] = WORLD_REV; // stale-but-boring
                        restSectKeeps++;
                    }
                    // itemCheckActive runs ONLY at merge-cadence ticks (its
                    // result gates the scan): on the other 39 ticks the
                    // inline arithmetic is identical for the active and
                    // inactive vanilla bodies, and checkIfActive's side
                    // effects are value-invisible for a resting item (the
                    // isTemporarilyActive toggle has no readers outside the
                    // skipped move path; MISC items take no immunity stamps)
                    boolean cadence = (e.tickCount + 1) % 40 == 0;
                    boolean active = cadence && itemCheckActive(e, true);
                    // dispatcher prologue mirror (the suppressed
                    // tickNonPassenger: setOldPosAndRot is value-identical
                    // at a fixed point, the counters are not)
                    e.tickCount = e.tickCount + 1;
                    e.totalEntityAge = e.totalEntityAge + 1;
                    // vanilla tick() order: pickupDelay--, merge cadence
                    // (active ticks only), age++, despawn. The merge scan
                    // runs INLINE (the exact L3 path) — a merge's own state
                    // changes are exact vanilla writes, so rest continues;
                    // an absorbed item keeps running the tail exactly like
                    // the vanilla body does.
                    boolean pdDec = false;
                    if (e.pickupDelay > 0 && e.pickupDelay != 32767) {
                        e.pickupDelay = e.pickupDelay - 1;
                        pdDec = true;
                    }
                    if (active && e.tickCount % 40 == 0) {
                        if (!mergeScanNative(e)) {
                            // index unsure: UNDO the mirror and let the
                            // vanilla body compute the whole tick (counters
                            // included) exactly — the resync re-arms rest
                            MODE_VAL[slot] = 0;
                            e.tickCount = e.tickCount - 1;
                            e.totalEntityAge = e.totalEntityAge - 1;
                            if (pdDec) {
                                e.pickupDelay = e.pickupDelay + 1;
                            }
                            dispPass++;
                            return false;
                        }
                    }
                    if (e.age != -32768) {
                        e.age = e.age + 1;
                    }
                    int rate = O_DESPAWN_RATE >= 0
                            ? U.getInt(e, O_DESPAWN_RATE)
                            : VH_DESPAWN_RATE.getInt(e);
                    if (e.age >= rate) {
                        despawnTail(e);
                    }
                    MODE_GEN[slot] = MODE_EPOCH; // keep the recency stamp alive
                    dispRest++;
                    return true;
                }
            }
            if (MODE_GEN[slot] != MODE_EPOCH || MODE_VAL[slot] == 0) {
                dispPass++;
                return false;
            }
            // cheap fresh-flag gates: always plain field reads
            // (stackEmptyDispatch: the cached empty bit replaces the
            // SynchedEntityData read — 6.55% MSPT on items-100k)
            if (e.isRemoved() || e.noPhysics
                    || e.portalProcess != null || e.isOnPortalCooldown()
                    || stackEmptyDispatch(e)) {
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
            boolean active = itemCheckActive(e, false);
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
    private static boolean itemCheckActive(ItemEntity e, boolean preIncrement) {
        // The tickCount the vanilla checkIfActive reads is the POST-commonTick
        // value (ServerLevel.tickNonPassenger: commonTick THEN tick, javap).
        // TWO callers, TWO phases: itemDispatch runs at ItemEntity.tick's
        // start — the field is ALREADY post (preIncrement=false); the rest
        // path fully suppresses tickNonPassenger and mirrors the ++ itself
        // AFTER the gate — its field is still pre (preIncrement=true). The
        // old unconditional "+1" shifted the dispatch phase by one: the MS
        // scan lists (run 40791707738, item 971) show the ON scan one tick
        // earlier than vanilla — the merge survivor flip at t=159.
        int tc = e.tickCount + (preIncrement ? 1 : 0);
        if ((tc + e.getId()) % 4 == 0) return true;
        if (e.defaultActivationState) return true;
        if (tc < 200) return true; // isAlive(): isRemoved gated above
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
        IB_FLAGS[i] = IF_NATIVE | (active ? 0 : IF_INACTIVE); // cd slot 0
        IB_REF[i] = e;
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

    // ---- L21 native rest ----
    // MODE_VAL == 2 means: the item's last native tick was a VALUE fixed
    // point (pos/delta/ground/fall identical, no events) — the dispatch
    // guard runs the exact counter arithmetic inline (tickCount,
    // totalEntityAge, pickupDelay, age, despawn, merge cadence) and the
    // item skips the whole collect -> flush -> apply cycle. REST_ID holds
    // the FULL entity id the stamp was proven for: entity ids are
    // monotonic per run, so a slot-colliding successor can never inherit
    // the fixed-point proof (the ARENA ref is kept as defense in depth).
    private static final long[] REST_ID = new long[1 << 22];
    // Per-section revision stamps: the flush drain stamps every invalidated
    // section with the post-bump WORLD_REV. A rest item whose environment
    // sections were not touched by the current revision keeps resting even
    // though the GLOBAL revision moved (a block change anywhere must not
    // herd every rest item through a vanilla tick).
    private static final int[] SECT_REV = new int[1 << 18];
    private static final int SECT_MASK = (1 << 18) - 1;
    public static volatile long dispRest = 0, restExits = 0, restSectKeeps = 0;

    private static int sectKeySlot(int sx, int sy, int sz) {
        return (int) ((((sx & 0x3FF) << 20) | ((sy & 0x3FF) << 10) | (sz & 0x3FF)) & SECT_MASK);
    }

    /** True when any section of the item's tick environment was touched by
     *  the CURRENT world revision. The envelope covers every block read the
     *  native tick makes: the box span (0.25 x 0.25 x 0.25), the swept box
     *  (gravity reaches 0.04 below), the friction/supporting block (one
     *  block below), and find_supporting_block's \u00b11 block scan margin.
     *  Only reached on a MODE_REV mismatch (a global block-change bump). */
    private static boolean restSectionTouched(ItemEntity e) {
        int rev = WORLD_REV;
        if (rev == 0) {
            return false;
        }
        double x = e.getX(), y = e.getY(), z = e.getZ();
        int bx0 = ((int) Math.floor(x - 0.125)) - 1;
        int bx1 = ((int) Math.floor(x + 0.125)) + 1;
        int by0 = ((int) Math.floor(y - 1.0)) - 1;
        int by1 = ((int) Math.floor(y + 0.25)) + 1;
        int bz0 = ((int) Math.floor(z - 0.125)) - 1;
        int bz1 = ((int) Math.floor(z + 0.125)) + 1;
        for (int sy = by0 >> 4; sy <= by1 >> 4; sy++) {
            for (int sx = bx0 >> 4; sx <= bx1 >> 4; sx++) {
                for (int sz = bz0 >> 4; sz <= bz1 >> 4; sz++) {
                    if (SECT_REV[sectKeySlot(sx, sy, sz)] == rev) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    // ---- L8.4 empty-stack dispatch cache ----
    // The tick guard must keep vanilla's FIRST-instruction parity:
    // ItemEntity.tick() starts with getItem().isEmpty() -> discard(DESPAWN)
    // (javap-verified on the shipped 1.21.8-60 jar), so empty-stack items
    // route vanilla. That check costs a SynchedEntityData read per item per
    // tick — measured 6.55% MSPT (721/11003 samples) on items-100k-mod.
    // EVERY stack write in ItemEntity goes through setItem (both ctors,
    // readAdditionalSaveData, merge absorb — javap: 4 call sites, all
    // invokevirtual setItem), so a prefix probe there keeps an O(1) byte
    // fresh. Values: 0 = never probed (fall back to the synched read),
    // 1 = non-empty, 2 = empty. Entity ids are globally monotonic per run,
    // and any genuinely-empty item was written empty through setItem, so a
    // probed slot cannot go stale in the empty->native direction.
    private static final byte[] STACK_EMPTY = new byte[1 << 22];

    /** Woven prefix probe on ItemEntity.setItem — refreshes the dispatch
     *  cache. Pure cache write: no mode changes, no wake, no state. */
    public static void probeSetItem(net.minecraft.world.entity.item.ItemEntity e,
            net.minecraft.world.item.ItemStack st) {
        try {
            STACK_EMPTY[e.getId() & MAMASK] = st.isEmpty() ? (byte) 2 : (byte) 1;
        } catch (Throwable ignored) {
            // cache is an optimization only; the dispatch falls back to the
            // synched read for unseen slots
        }
    }

    /** Empty-stack parity check for the dispatch guard: cached bit when the
     *  item has been seen by the setItem probe, synched read otherwise. */
    private static boolean stackEmptyDispatch(net.minecraft.world.entity.item.ItemEntity e) {
        byte v = STACK_EMPTY[e.getId() & MAMASK];
        return v == 0 ? e.getItem().isEmpty() : v == (byte) 2;
    }

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
    // ===================================================================
    // L16 — the Unsafe floor. The L15 profile (b0a2095) put 12.86% of the
    // items-100k leg INSIDE java.lang.invoke.VarHandleReferences — every
    // RefField read paid a megamorphic VarHandle.invoke chain on top of the
    // actual field load. sun.misc.Unsafe with a captured field offset
    // compiles to the getfield itself: ~1-2ns vs ~15-30ns, zero extra
    // frames. The VarHandle/reflection paths stay as the fallback for a
    // JVM that refuses theUnsafe (behaviour identical, just slower).
    // ===================================================================
    private static final sun.misc.Unsafe U;
    static {
        sun.misc.Unsafe u = null;
        try {
            java.lang.reflect.Field f =
                    sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            u = (sun.misc.Unsafe) f.get(null);
        } catch (Throwable t) {
            u = null; // fallback: VarHandle / reflection below
        }
        U = u;
    }

    /** Direct-offset read/write helpers for fields captured outside
     *  RefField (the tracker guard's Entity-side flags). All callers hold
     *  the offset from offOf() at arm time; a null U never reaches them
     *  (offOf throws instead). */
    private static long offOf(Class<?> start, String name) {
        Class<?> k = start;
        while (k != null) {
            try {
                java.lang.reflect.Field f = k.getDeclaredField(name);
                f.setAccessible(true);
                return U.objectFieldOffset(f);
            } catch (NoSuchFieldException notHere) {
                k = k.getSuperclass();
            } catch (Throwable t) {
                break;
            }
        }
        throw new IllegalStateException("offOf: " + start.getName() + "." + name);
    }
    private static Object refAt(Object o, long off) { return U.getObject(o, off); }
    private static void putRefAt(Object o, long off, Object v) { U.putObject(o, off, v); }
    private static int intAt(Object o, long off) { return U.getInt(o, off); }
    private static void putIntAt(Object o, long off, int v) { U.putInt(o, off, v); }

    // L19: the DataItem fast read. SynchedEntityData.getItem (javap, 26.3)
    // is exactly `itemsById[accessor.id()]` and get() adds only the value
    // load — with the two field offsets captured once, a gate reads the
    // CURRENT value in two plain loads instead of the full get() chain
    // (the accessor lookup + boxed return + exception-table frames that
    // made the conv gate cost MORE than the vanilla write it replaced).
    private static long O_ED_ITEMS = -1, O_DI_VALUE = -1;
    // L20 quiet-travel floor: the protected stuckSpeedMultiplier offset
    // (the input vector rides the op-5 guard's second argument — no field
    // offsets needed for xxa/yya/zza)
    private static long O_STUCK = -1;
    // L20b: the rest branch's despawnRate read (valid when >= 0)
    private static long O_DESPAWN_RATE = -1;
    // L22b: the inside-block-effect scratch lists (ArrayDeque / List)
    private static long O_MOVEMENT_TICK = -1, O_FINAL_MOVEMENTS = -1;

    // ---- L22b: the inside-block-effect no-op floor ----
    // Woven EarlyReturnGuard on Entity.applyEffectsFromBlocks(). Vanilla
    // walks every block position the swept hull intersects, runs
    // getEntityInsideCollisionShape + fluid tests per non-air position and
    // steps the effect collector — all of which is a PROVEN value no-op
    // when every visited position is AIR (the lambda's own isAir
    // short-circuit makes air positions do nothing), the below block's
    // stepOn is the empty base implementation, the mob is not in rain, the
    // fire/freezing pre-states are false and the tail's
    // setRemainingFireTicks(-immunity) rewrite would be same-value. The
    // hull enumeration is a strict SUPERSET of the vanilla visit set
    // (touching positions included), so "superset all-air" implies the
    // vanilla visit set is all-air — the gate fails safe to vanilla.
    // Scope: ALL ENTITIES. The 2026-10-25 crash pair (items-100k @ c2140cf,
    // mixed-150k @ e64e66f, hs_err 4417+5536) was FIRST blamed on the item
    // path, and e64e66f mob-scoped the gate as a precaution. The e64e66f
    // mixed-150k leg THEN crashed with the item path OFF — the item path is
    // exonerated; the only factor common to all four L22-era crashes is the
    // L20b SE_FAST direct-offset tracker path (ON in every crash run,
    // OFF in every green run before AND after — see the trackerDispatch
    // comment). The item path is re-enabled by default with its own kill
    // switch (CE_BLOCK_FX_ITEMS=0) for the bisect.
    public static final boolean BLOCK_FX_ON =
            !"0".equals(System.getenv("CE_BLOCK_FX"));
    public static final boolean BLOCK_FX_ITEMS =
            !"0".equals(System.getenv("CE_BLOCK_FX_ITEMS"));
    public static long blockFxChecks = 0, blockFxQuiet = 0, blockFxFail = 0;
    // Block class -> stepOn is the empty base implementation (probed once
    // via getMethod().getDeclaringClass() — the exact override test)
    private static final java.util.concurrent.ConcurrentHashMap<Class<?>, Boolean>
            STEP_ON_BORING = new java.util.concurrent.ConcurrentHashMap<>();

    private static boolean stepOnIsNoOp(net.minecraft.world.level.block.Block b) {
        return STEP_ON_BORING.computeIfAbsent(b.getClass(), k -> {
            try {
                return k.getMethod("stepOn",
                        net.minecraft.world.level.Level.class,
                        net.minecraft.core.BlockPos.class,
                        net.minecraft.world.level.block.state.BlockState.class,
                        net.minecraft.world.entity.Entity.class
                ).getDeclaringClass() == net.minecraft.world.level.block.Block.class;
            } catch (Throwable t) {
                return Boolean.FALSE;
            }
        });
    }

    public static boolean mobBlockEffectsNoOp(net.minecraft.world.entity.Entity e) {
        if (!ACTIVE || !BLOCK_FX_ON || O_MOVEMENT_TICK < 0) return false;
        if (!BLOCK_FX_ITEMS && !(e instanceof net.minecraft.world.entity.Mob)) return false;
        blockFxChecks++;
        try {
            if (e.isRemoved() || e.noPhysics || !e.isAlive()) return false;
            net.minecraft.world.level.Level lvl = e.level();
            if (lvl.isClientSide()) return false;
            // recorded movements this tick (portals, teleports) → the
            // vanilla movement list path, not our single-sweep shape
            Object mv = U.getObject(e, O_MOVEMENT_TICK);
            if (mv instanceof java.util.Collection<?> c && !c.isEmpty()) return false;
            // debug subscribers observe per-position intersections
            if (lvl instanceof net.minecraft.server.level.ServerLevel sl
                    && sl.getServer().debugSubscribers()
                        .hasAnySubscriberFor(net.minecraft.util.debug.DebugSubscriptions
                            .ENTITY_BLOCK_INTERSECTIONS)) {
                return false;
            }
            net.minecraft.world.phys.AABB box = e.getBoundingBox();
            double dx = e.getX() - e.xo, dy = e.getY() - e.yo, dz = e.getZ() - e.zo;
            // hull of the from-box and the to-box (contains the swept
            // segment; a superset of both vanilla visit sources)
            double hx0 = Math.min(box.minX, box.minX + dx);
            double hx1 = Math.max(box.maxX, box.maxX + dx);
            double hy0 = Math.min(box.minY, box.minY + dy);
            double hy1 = Math.max(box.maxY, box.maxY + dy);
            double hz0 = Math.min(box.minZ, box.minZ + dz);
            double hz1 = Math.max(box.maxZ, box.maxZ + dz);
            int x0 = net.minecraft.util.Mth.floor(hx0);
            int x1 = net.minecraft.util.Mth.floor(hx1);
            int y0 = net.minecraft.util.Mth.floor(hy0);
            int y1 = net.minecraft.util.Mth.floor(hy1);
            int z0 = net.minecraft.util.Mth.floor(hz0);
            int z1 = net.minecraft.util.Mth.floor(hz1);
            // 4^3 cap: hulls beyond 64 positions route vanilla (the gate
            // must stay cheaper than the thing it replaces)
            if ((x1 - x0 + 1) * (y1 - y0 + 1) * (z1 - z0 + 1) > 64) return false;
            for (int bx = x0; bx <= x1; bx++) {
                for (int by = y0; by <= y1; by++) {
                    for (int bz = z0; bz <= z1; bz++) {
                        if (!lvl.getBlockState(new net.minecraft.core.BlockPos(bx, by, bz)).isAir()) {
                            return false;
                        }
                    }
                }
            }
            // the onGround stepOn probe — exact vanilla position
            if (e.onGround()) {
                net.minecraft.core.BlockPos below;
                if (e.mainSupportingBlockPos.isPresent()
                        && lvl.getChunkIfLoadedImmediately(e.mainSupportingBlockPos.get()) != null) {
                    below = e.mainSupportingBlockPos.get()
                            .atY(net.minecraft.util.Mth.floor(e.getY() - 0.2F));
                } else {
                    below = new net.minecraft.core.BlockPos(
                            net.minecraft.util.Mth.floor(e.getX()),
                            net.minecraft.util.Mth.floor(e.getY() - 0.2F),
                            net.minecraft.util.Mth.floor(e.getZ()));
                }
                if (!stepOnIsNoOp(lvl.getBlockState(below).getBlock())) return false;
            }
            // rain / fire / freezing pre-states
            if (e.isInRain() || e.isOnFire() || e.isFreezing()) return false;
            // the tail's same-value rewrite must already hold
            if (e.getRemainingFireTicks() != -e.getFireImmuneTicks()) return false;
            // replicate the scratch-list maintenance the no-arg body does:
            // final.clear() + add(default movement). The add is dropped —
            // with an empty final list a later
            // applyEffectsFromBlocksForLastMovements() iterates nothing,
            // which is observationally identical to iterating the default
            // movement: our gate PROVED the swept hull is all-air (zero
            // collector effects) and the tail rewrite is same-value.
            Object fin = U.getObject(e, O_FINAL_MOVEMENTS);
            if (fin instanceof java.util.List<?> fl) {
                ((java.util.List) fl).clear();
            }
            blockFxQuiet++;
            return true;
        } catch (Throwable t) {
            blockFxFail++;
            lastError = System.nanoTime();
            lastErrorText = "mobBlockEffectsNoOp: " + t;
            return false;
        }
    }

    // ---- L20: the quiet-mob travel floor ----
    // Woven EarlyReturnGuard on LivingEntity.travel(Vec3). When the
    // predicate below holds, the ENTIRE vanilla travel slice is provably a
    // VALUE no-op for this tick: moveRelative adds Vec3.ZERO (the input
    // length short-circuit), handleOnClimbable is skipped (not climbable),
    // the collide clips the swept move to exactly (0,0,0) (full-block
    // ground at box.minY, no hard entity colliders, inside the border),
    // the restitution zeroes the bounce (|delta.y| <= effectiveGravity),
    // and the drag/gravity tail recomputes the identical delta. Every
    // field vanilla writes equals the pre-tick value, so skipping the
    // slice is value-exact. The proof is STATELESS: every input is re-read
    // per tick, so any block/attribute/flag change fails the predicate and
    // routes vanilla (self-healing, no mode tables, no flush, no Rust).
    //
    // The delta check is also the omni/discardFriction discriminator: a
    // mob whose verticalFriction differs (omnidirectionalAirMover, or
    // shouldDiscardFriction) has a DIFFERENT fixed-point delta, so the
    // exact-equality test fails and those mobs route vanilla.
    public static final boolean MOB_TRAVEL_ON =
            !"0".equals(System.getenv("CE_MOB_TRAVEL"));
    // plain longs (the L15b lesson: a volatile add per entity per tick was
    // 1-2ms/tick at 100k; the verifier/e2e readers tolerate eventual
    // visibility)
    public static long travelChecks = 0, travelQuiet = 0, travelWalk = 0, travelFail = 0;
    // L24b: the two protected members the SHAPE C writeback needs —
    // resolved once with setAccessible (unreflect honors the flag), the
    // virtual dispatch stays exact through the MethodHandle
    private static volatile MethodHandle MH_BACK_OFF_EDGE, MH_BOUNCINESS;
    private static volatile boolean TRAVEL_MH_READY = false;

    private static synchronized boolean initTravelMh() {
        if (TRAVEL_MH_READY) return true;
        try {
            java.lang.reflect.Method bo = net.minecraft.world.entity.Entity.class
                    .getDeclaredMethod("maybeBackOffFromEdge",
                            net.minecraft.world.phys.Vec3.class,
                            net.minecraft.world.entity.MoverType.class);
            bo.setAccessible(true);
            MH_BACK_OFF_EDGE = MethodHandles.lookup().unreflect(bo);
            java.lang.reflect.Method bn = net.minecraft.world.entity.LivingEntity.class
                    .getDeclaredMethod("getEntityBounciness");
            bn.setAccessible(true);
            MH_BOUNCINESS = MethodHandles.lookup().unreflect(bn);
            TRAVEL_MH_READY = MH_BACK_OFF_EDGE != null && MH_BOUNCINESS != null;
            return TRAVEL_MH_READY;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initTravelMh: " + t;
            return false;
        }
    }

    // L27: the per-mob AttributeInstance cache. AttributeMap builds its
    // instances once in the constructor and never replaces them, so caching
    // the instance references is the same read as AttributeMap.getValue
    // (which is attributes.get(holder) -> instance.getValue()) without the
    // HashMap.get per call (the 2.18% AttributeMap.getInstance bucket in
    // the mixed-150k profile; the SHAPE C path pays it 3x per covered mob
    // per tick). Slots: [0]=AIR_DRAG_MODIFIER, [1]=FRICTION_MODIFIER,
    // [2]=MOVEMENT_SPEED. A null slot means the supplier lacks the
    // attribute — vanilla's getAttributeValue base-default path routes for
    // those. Weak-keyed identity table (the AIKey pattern): an entry dies
    // with its mob, dead slots are claimed in place, server-thread confined
    // (mobTravelNative runs inside the tick phase).
    private static final int AT_CAP_INIT = 1 << 13;   // 8192 slots
    private static AIKey[] AT_KEYS = new AIKey[AT_CAP_INIT];
    private static net.minecraft.world.entity.ai.attributes.AttributeInstance[][] AT_VALS =
            new net.minecraft.world.entity.ai.attributes.AttributeInstance[AT_CAP_INIT][];
    private static int AT_SIZE = 0, AT_DEAD = 0;

    private static net.minecraft.world.entity.ai.attributes.AttributeInstance[] attrCache(
            net.minecraft.world.entity.LivingEntity e) {
        AIKey[] keys = AT_KEYS;
        net.minecraft.world.entity.ai.attributes.AttributeInstance[][] vals = AT_VALS;
        int mask = keys.length - 1;
        int i = System.identityHashCode(e) & mask;
        while (true) {
            AIKey k = keys[i];
            if (k == null) {
                net.minecraft.world.entity.ai.attributes.AttributeMap am = e.getAttributes();
                net.minecraft.world.entity.ai.attributes.AttributeInstance[] v =
                        new net.minecraft.world.entity.ai.attributes.AttributeInstance[]{
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.AIR_DRAG_MODIFIER),
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.FRICTION_MODIFIER),
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.MOVEMENT_SPEED),
                        };
                keys[i] = new AIKey(e);
                vals[i] = v;
                AT_SIZE++;
                if (AT_SIZE + (AT_SIZE >> 1) >= keys.length) {
                    attrRebuild(keys.length << 1);
                }
                return v;
            }
            Object cur = k.get();
            if (cur == e) return vals[i];
            if (cur == null) {
                // dead slot: claim in place (same linear-probe law as the
                // AI table — the incoming key's probe path starts here)
                net.minecraft.world.entity.ai.attributes.AttributeMap am = e.getAttributes();
                net.minecraft.world.entity.ai.attributes.AttributeInstance[] v =
                        new net.minecraft.world.entity.ai.attributes.AttributeInstance[]{
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.AIR_DRAG_MODIFIER),
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.FRICTION_MODIFIER),
                                am.getInstance(net.minecraft.world.entity.ai.attributes
                                        .Attributes.MOVEMENT_SPEED),
                        };
                AT_DEAD++;
                keys[i] = new AIKey(e);
                vals[i] = v;
                if (AT_DEAD > 2048 && AT_DEAD * 2 > AT_SIZE) {
                    attrRebuild(keys.length); // sweep dead, same capacity
                }
                return v;
            }
            i = (i + 1) & mask;
        }
    }

    private static void attrRebuild(int newCap) {
        AIKey[] oldKeys = AT_KEYS;
        net.minecraft.world.entity.ai.attributes.AttributeInstance[][] oldVals = AT_VALS;
        AIKey[] nk = new AIKey[newCap];
        net.minecraft.world.entity.ai.attributes.AttributeInstance[][] nv =
                new net.minecraft.world.entity.ai.attributes.AttributeInstance[newCap][];
        int mask = newCap - 1;
        int live = 0;
        for (int j = 0; j < oldKeys.length; j++) {
            AIKey k = oldKeys[j];
            if (k == null) continue;
            if (k.get() == null) continue; // dead — dropped
            int i = k.hash & mask;
            while (nk[i] != null) i = (i + 1) & mask;
            nk[i] = k;
            nv[i] = oldVals[j];
            live++;
        }
        AT_KEYS = nk;
        AT_VALS = nv;
        AT_SIZE = live;
        AT_DEAD = 0;
    }

    public static boolean mobTravelNative(net.minecraft.world.entity.LivingEntity e,
            net.minecraft.world.phys.Vec3 input) {
        if (!ACTIVE || !MOB_TRAVEL_ON || O_STUCK < 0) return false;
        travelChecks++;
        try {
            if (e.level().isClientSide() || e.isRemoved()
                    || input.x != 0.0F || input.y != 0.0F || input.z != 0.0F
                    || !e.getPassengers().isEmpty()
                    || e.isFallFlying()
                    || e.isTemporarilyActive
                    || !e.getActiveEffects().isEmpty()) {
                return false;
            }
            net.minecraft.world.phys.Vec3 d = e.getDeltaMovement();
            if (d.x == 0.0 && d.y == 0.0 && d.z == 0.0) {
                // SHAPE A: the zero-delta fixed point (noGravity mobs).
                // The authoritative flag block sets onGround=false and the
                // three collision flags false every tick; the pre-state
                // must already equal that. checkFallDamage leaves fall
                // untouched (any value is fixed) but the climbable clamp
                // would reset it AND write lastClimbablePos -> excluded.
                if (e.getGravity() != 0.0
                        || e.onGround()
                        || e.mainSupportingBlockPos.isPresent()
                        || e.horizontalCollision
                        || e.verticalCollision
                        || e.verticalCollisionBelow
                        || e.minorHorizontalCollision
                        || e.fallDistance != 0.0
                        || e.onClimbable()) {
                    return false;
                }
                travelQuiet++;
                return true;
            }
            // SHAPE C — the moving grounded walk on proven-open ground (the
            // L24b port; the delta math is the bit-exact mirror of the
            // javap'd 26.3 travelInAir chain and is pinned by
            // centities-core/src/mobtravel.rs). Covers the dominant moving
            // shape: a grounded untracked mob walking across open ground.
            // Vanilla pays: 4 attribute reads, the friction block read, the
            // moveRelative, the full collide (block shapes + hard colliders
            // + step-up) and the flag bookkeeping. We compute the delta
            // natively and skip the collide by PROVING the swept hull is
            // air and the ground layer is full zero-bounce blocks under the
            // union footprint.
            if (d.x != 0.0 || d.z != 0.0) {
                // NOTE on the 0.2 powder-snow/climb branch (bc 41-91 of
                // handleRelativeFriction): it fires only when
                // (horizontalCollision || jumping) && (onClimbable ||
                // (wasInPowderSnow && canWalkOnPowderSnow)). The routing
                // proves horizontalCollision false (the hull is open),
                // requires wasInPowderSnow false, and the hull proof makes
                // onClimbable provably false — so the branch cannot fire
                // regardless of `jumping`, and the protected jump field
                // needs no read. isImmobile is likewise input-side only:
                // aiStep zeroes the input before travel sees it, so the
                // captured input already reflects it.
                if (input.y != 0.0F || d.y >= -1.0e-7
                        || e.wasInPowderSnow
                        || e.horizontalCollision || e.minorHorizontalCollision
                        || e.onClimbable() || e.isPassenger()
                        || e.getVehicle() != null
                        || e.isInWater() || e.isInLava()) {
                    return false;
                }
                net.minecraft.world.level.Level lvl = e.level();
                // the steady-walker vertical fixed point: the previous
                // tail wrote delta.y = -gravity * computeModifiedFriction(
                // 0.98, drag) — the same shape SHAPE B pins
                double y = e.getY();
                if (y != Math.floor(y)) return false; // the ground-clip exactness
                double g = e.getGravity(); // = getEffectiveGravity() (virtual)
                if (g <= 0.0) return false;
                // L27: the three instances come from the per-mob cache —
                // getAttributeValue paid a HashMap.get per call (the map is
                // per-entity and the instances are built once in the
                // AttributeMap ctor and never replaced), so caching the
                // instance and calling getValue() directly is the same read
                // without the lookup. A null instance (an attribute absent
                // from the supplier — vanilla returns the base default) is
                // cached too and routes getAttributeValue.
                net.minecraft.world.entity.ai.attributes.AttributeInstance[] attrs =
                        attrCache(e);
                float dragMod = attrs[0] != null
                        ? (float) attrs[0].getValue()
                        : (float) e.getAttributeValue(
                                net.minecraft.world.entity.ai.attributes.Attributes.AIR_DRAG_MODIFIER);
                float vf = 1.0F - (1.0F - 0.98F) * dragMod; // computeModifiedFriction
                if (vf < 0.0F) vf = 0.0F;
                if (vf > 1.0F) vf = 1.0F;
                if (d.y != -g * (double) vf) return false;
                Object stuckC = U.getObject(e, O_STUCK);
                if (stuckC instanceof net.minecraft.world.phys.Vec3 svC
                        && svC.x * svC.x + svC.y * svC.y + svC.z * svC.z > 1.0e-7) {
                    return false;
                }
                // the untracked contract: with no recipients the move()'s
                // sound/gameEvent tails are unobservable (sound range 16 <<
                // mob tracking range), which is what makes the skip exact.
                // moonrise$getTrackedEntity returns the ChunkMap$TrackedEntity
                // (NOT the ServerEntity!) — the recipients are ITS seenBy set
                // (the TrackedEntity IS the ServerEntity$Synchronizer), and
                // moonrise$hasPlayers() is the exact public query. The first
                // e2e crash (hs_err 4006) was a raw ServerEntity-offset read
                // against a TrackedEntity instance — the same wild-oop bug
                // class that killed the SE_FAST path; typed access only now.
                Object trkC = e.moonrise$getTrackedEntity();
                if (trkC != null
                        && ((ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerTrackedEntity) trkC)
                                .moonrise$hasPlayers()) {
                    return false;
                }
                // the delta math (bit-exact; mobtravel.rs pins the floats)
                float fmC = attrs[1] != null
                        ? (float) attrs[1].getValue()
                        : (float) e.getAttributeValue(
                                net.minecraft.world.entity.ai.attributes.Attributes.FRICTION_MODIFIER);
                float speedC = attrs[2] != null
                        ? (float) attrs[2].getValue()
                        : (float) e.getAttributeValue(
                                net.minecraft.world.entity.ai.attributes.Attributes.MOVEMENT_SPEED);
                float bfC = lvl.getBlockState(
                        e.getBlockPosBelowThatAffectsMyMovement()).getBlock().getFriction();
                float frictC = 1.0F - (1.0F - bfC) * fmC; // computeModifiedFriction
                if (frictC < 0.0F) frictC = 0.0F;
                if (frictC > 1.0F) frictC = 1.0F;
                float fisC = frictC > 0.6F
                        ? speedC * (0.21600002F / (frictC * frictC * frictC))
                        : speedC;
                double lsC = (double) input.x * (double) input.x
                        + (double) input.y * (double) input.y
                        + (double) input.z * (double) input.z;
                double dmX = d.x, dmY = d.y, dmZ = d.z;
                if (lsC >= 1.0e-7) {
                    double nx, ny, nz;
                    if (lsC > 1.0) {
                        double len = Math.sqrt(lsC);
                        nx = (double) input.x / len;
                        ny = (double) input.y / len;
                        nz = (double) input.z / len;
                    } else {
                        nx = (double) input.x;
                        ny = (double) input.y;
                        nz = (double) input.z;
                    }
                    nx *= (double) fisC;
                    ny *= (double) fisC;
                    nz *= (double) fisC;
                    float yrC = e.getYRot() * 0.017453292F;
                    float sinC = net.minecraft.util.Mth.sin((double) yrC);
                    float cosC = net.minecraft.util.Mth.cos((double) yrC);
                    dmX += nx * (double) cosC - nz * (double) sinC;
                    dmY += ny;
                    dmZ += nz * (double) cosC + nx * (double) sinC;
                }
                // maybeBackOffFromEdge must be the identity (non-sneaking
                // mobs) — compare by reference like the no-op returns
                if (!initTravelMh()) return false;
                net.minecraft.world.phys.Vec3 wantedC =
                        new net.minecraft.world.phys.Vec3(dmX, dmY, dmZ);
                if (MH_BACK_OFF_EDGE.invoke(e, wantedC,
                        net.minecraft.world.entity.MoverType.SELF) != wantedC) {
                    return false;
                }
                // --- the hull proof ---
                net.minecraft.world.phys.AABB boxC = e.getBoundingBox();
                double hy0C = boxC.minY + dmY; // dmY < 0: the swept bottom
                double hx0C = Math.min(boxC.minX, boxC.minX + dmX);
                double hx1C = Math.max(boxC.maxX, boxC.maxX + dmX);
                double hz0C = Math.min(boxC.minZ, boxC.minZ + dmZ);
                double hz1C = Math.max(boxC.maxZ, boxC.maxZ + dmZ);
                int gyC = (int) y - 1;
                if (net.minecraft.util.Mth.floor(hy0C) != gyC) return false;
                // the body layers (a SUPERSET of the collide's strict set:
                // the ±1e-7 outer margin can only route vanilla more often)
                int ax0 = net.minecraft.util.Mth.floor(hx0C - 1.0e-7);
                int ax1 = net.minecraft.util.Mth.floor(hx1C + 1.0e-7);
                int az0 = net.minecraft.util.Mth.floor(hz0C - 1.0e-7);
                int az1 = net.minecraft.util.Mth.floor(hz1C + 1.0e-7);
                int ay0 = gyC + 1;
                int ay1 = net.minecraft.util.Mth.floor(boxC.maxY - 1.0e-7);
                if ((ax1 - ax0 + 1) * (long) (az1 - az0 + 1)
                        * (ay1 - ay0 + 1) > 4096L) {
                    return false;
                }
                if (((double) MH_BOUNCINESS.invoke(e)) != 0.0) return false;
                for (int bx = ax0; bx <= ax1; bx++) {
                    for (int bz = az0; bz <= az1; bz++) {
                        for (int by = ay0; by <= ay1; by++) {
                            if (!lvl.getBlockState(
                                    new net.minecraft.core.BlockPos(bx, by, bz)).isAir()) {
                                return false;
                            }
                        }
                    }
                }
                // the ground layer: full zero-bounce blocks under the
                // UNION footprint (the touch margins mirror the gather)
                int gx0 = net.minecraft.util.Mth.floor(hx0C + 1.0e-7);
                int gx1 = net.minecraft.util.Mth.floor(hx1C - 1.0e-7);
                int gz0 = net.minecraft.util.Mth.floor(hz0C + 1.0e-7);
                int gz1 = net.minecraft.util.Mth.floor(hz1C - 1.0e-7);
                for (int bx = gx0; bx <= gx1; bx++) {
                    for (int bz = gz0; bz <= gz1; bz++) {
                        net.minecraft.core.BlockPos bpC =
                                new net.minecraft.core.BlockPos(bx, gyC, bz);
                        net.minecraft.world.level.block.state.BlockState stC =
                                lvl.getBlockState(bpC);
                        if (!stC.isCollisionShapeFullBlock(lvl, bpC)
                                || stC.getBlock().getBounceRestitution() != 0.0F) {
                            return false;
                        }
                    }
                }
                net.minecraft.world.phys.AABB sweptC = new net.minecraft.world.phys.AABB(
                        hx0C, hy0C, hz0C, hx1C, boxC.maxY, hz1C);
                java.util.List<net.minecraft.world.phys.AABB> hardC =
                        new java.util.ArrayList<>(2);
                ca.spottedleaf.moonrise.patches.collisions.CollisionUtil
                        .getEntityHardCollisions(lvl, e, sweptC, hardC, 0, null);
                if (!hardC.isEmpty()) return false;
                net.minecraft.world.level.border.WorldBorder wbC = lvl.getWorldBorder();
                if (boxC.minX + dmX <= wbC.getMinX() || boxC.maxX + dmX >= wbC.getMaxX()
                        || boxC.minZ + dmZ <= wbC.getMinZ() || boxC.maxZ + dmZ >= wbC.getMaxZ()) {
                    return false;
                }
                // --- the writeback (the exact vanilla order) ---
                // move(): setPos(position + clipped); recordMovement;
                // horizontalCollision=false (proven equal); vertical
                // Collision=true / below=true (same-value writes — the
                // pre-state is required); setOnGroundWithMovement — the
                // REAL Level supporting-block rescan (exact by
                // construction); checkFallDamage(0.0, true, ..) is a
                // proven no-op (fallDistance==0, moved dy==0.0). The
                // restitution zeroed dmY. The travel tail applies the drag
                // to the post-restitution delta. addMovementThisTick is
                // NOT reproduced: for mobs the deque is consumed only by
                // applyEffectsFromBlocks, which the block-fx gate no-ops
                // on the SAME all-air hull — the record is unobservable,
                // and skipping it keeps that gate armed.
                e.setPos(e.getX() + dmX, y, e.getZ() + dmZ);
                net.minecraft.world.phys.Vec3 clippedC =
                        new net.minecraft.world.phys.Vec3(dmX, 0.0, dmZ);
                e.recordMovement(net.minecraft.world.entity.MoverType.SELF, clippedC);
                e.setOnGroundWithMovement(true, false, clippedC);
                float hdC = 1.0F - (1.0F - 0.91F) * dragMod; // the horizontal drag
                if (hdC < 0.0F) hdC = 0.0F;
                if (hdC > 1.0F) hdC = 1.0F;
                float hC = frictC * hdC;
                e.setDeltaMovement(new net.minecraft.world.phys.Vec3(
                        dmX * (double) hC,
                        (0.0 - g) * (double) vf,
                        dmZ * (double) hC));
                travelWalk++;
                return true;
            }
            // SHAPE B: the standing fixed point (0, -g * vf, 0)
            if (d.x != 0.0 || d.z != 0.0 || !e.onGround()
                    || e.fallDistance != 0.0
                    || e.horizontalCollision
                    || !e.verticalCollision
                    || !e.verticalCollisionBelow
                    || e.minorHorizontalCollision
                    || !e.mainSupportingBlockPos.isPresent()
                    || e.onClimbable()
                    || e.wasInPowderSnow) {
                return false;
            }
            double y = e.getY();
            if (y != Math.floor(y)) return false; // the clip must be exact
            // the swept slab must REACH the ground layer for the collide to
            // clip it (the gather's EPS7 margin): |d.y| > 1e-7 else the move
            // passes unclipped and the mob would sink
            if (d.y >= -1.0e-7) return false;
            double g = e.getGravity();
            if (g <= 0.0) return false;
            float dragMod =
                    (float) e.getAttributeValue(
                            net.minecraft.world.entity.ai.attributes.Attributes.AIR_DRAG_MODIFIER);
            float vf = 1.0F - (1.0F - 0.98F) * dragMod; // computeModifiedFriction
            if (vf < 0.0F) vf = 0.0F;
            if (vf > 1.0F) vf = 1.0F;
            if (d.y != -g * (double) vf) return false;
            Object stuck = U.getObject(e, O_STUCK);
            if (stuck instanceof net.minecraft.world.phys.Vec3 sv
                    && sv.x * sv.x + sv.y * sv.y + sv.z * sv.z > 1.0e-7) {
                return false;
            }
            // travel's own router: the fluid branch must not be taken
            net.minecraft.world.level.material.FluidState fs =
                    e.level().getFluidState(e.blockPosition());
            if (e.isInLiquid() && e.isAffectedByFluids() && !e.canStandOnFluid(fs)) {
                return false;
            }
            // every footprint column at the ground layer must be a FULL
            // block: the swept down-move then clips to exactly zero. The
            // column range mirrors the gather's EPS7 intersect margins
            // (touching columns never gather).
            net.minecraft.world.phys.AABB box = e.getBoundingBox();
            int gy = (int) y - 1;
            int x0 = net.minecraft.util.Mth.floor(box.minX + 1.0e-7);
            int x1 = net.minecraft.util.Mth.floor(box.maxX - 1.0e-7);
            int z0 = net.minecraft.util.Mth.floor(box.minZ + 1.0e-7);
            int z1 = net.minecraft.util.Mth.floor(box.maxZ - 1.0e-7);
            net.minecraft.world.level.Level lvl = e.level();
            for (int bx = x0; bx <= x1; bx++) {
                for (int bz = z0; bz <= z1; bz++) {
                    net.minecraft.core.BlockPos bp =
                            new net.minecraft.core.BlockPos(bx, gy, bz);
                    if (!lvl.getBlockState(bp).isCollisionShapeFullBlock(lvl, bp)) {
                        return false;
                    }
                }
            }
            // no hard entity colliders in the swept slab (the same query
            // the vanilla collide runs), strictly inside the border
            net.minecraft.world.phys.AABB swept = new net.minecraft.world.phys.AABB(
                    box.minX, box.minY + d.y, box.minZ, box.maxX, box.minY, box.maxZ);
            java.util.List<net.minecraft.world.phys.AABB> hard =
                    new java.util.ArrayList<>(2);
            ca.spottedleaf.moonrise.patches.collisions.CollisionUtil
                    .getEntityHardCollisions(lvl, e, swept, hard, 0, null);
            if (!hard.isEmpty()) return false;
            net.minecraft.world.level.border.WorldBorder wb = lvl.getWorldBorder();
            if (box.minX <= wb.getMinX() || box.maxX >= wb.getMaxX()
                    || box.minZ <= wb.getMinZ() || box.maxZ >= wb.getMaxZ()) {
                return false;
            }
            travelQuiet++;
            return true;
        } catch (Throwable t) {
            travelFail++;
            lastError = System.nanoTime();
            lastErrorText = "mobTravelNative: " + t;
            return false;
        }
    }

    private static void initDataOffsets() {
        // L27 FIX (e2e 9474477): the O_ACC_ID resolution must NOT live behind
        // the O_ED_ITEMS early-return — initDataOffsets first ran at the L15
        // frozen-gate init, before O_ACC_ID existed, so every later call
        // returned on the guard and the accessor-id offset stayed -1. That
        // made entityDataValue() return null on EVERY call and all three
        // same-value gates routed vanilla (the frozen gate served zero
        // no-ops; the assert caught it). Resolve idempotently BEFORE the
        // guard.
        if (O_ACC_ID < 0 && U != null) {
            try {
                O_ACC_ID = U.objectFieldOffset(
                        net.minecraft.network.syncher.EntityDataAccessor.class
                                .getDeclaredField("id"));
            } catch (Throwable t) {
                O_ACC_ID = -1; // the typed gates fall back to the get() chain
            }
        }
        if (O_ED_ITEMS >= 0 || U == null) return;
        try {
            O_ED_ITEMS = U.objectFieldOffset(
                    net.minecraft.network.syncher.SynchedEntityData.class
                            .getDeclaredField("itemsById"));
            O_DI_VALUE = U.objectFieldOffset(
                    Class.forName(
                            "net.minecraft.network.syncher.SynchedEntityData$DataItem")
                            .getDeclaredField("value"));
            // L20 quiet-travel floor: the protected stuck multiplier offset
            O_STUCK = offOf(net.minecraft.world.entity.Entity.class,
                    "stuckSpeedMultiplier");
            // L22b: the inside-block-effect scratch lists
            O_MOVEMENT_TICK = offOf(net.minecraft.world.entity.Entity.class,
                    "movementThisTick");
            O_FINAL_MOVEMENTS = offOf(net.minecraft.world.entity.Entity.class,
                    "finalMovementsThisTick");
        } catch (Throwable t) {
            O_ED_ITEMS = -1; // gates fall back to the get() chain
        }
    }

    /** Current value of `accessor` on `ed` — the get() fast path. Only
     *  valid when O_ED_ITEMS >= 0 (the caller checks). */
    private static Object dataValueAt(net.minecraft.network.syncher.SynchedEntityData ed,
                                      Object accessor) {
        Object[] items = (Object[]) refAt(ed, O_ED_ITEMS);
        int idx = ((net.minecraft.network.syncher.EntityDataAccessor<?>) accessor).id();
        return refAt(items[idx], O_DI_VALUE);
    }

    private static final class RefField {
        final java.lang.reflect.Field f;
        final VarHandle vh;
        /** L16: the Unsafe field offset — the fast path for every read and
         *  write below. -1 only when U is null (the fallback JVM). */
        final long off;
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
            long o = -1L;
            if (U != null) {
                try {
                    o = U.objectFieldOffset(found);
                } catch (Throwable uRefused) {
                    o = -1L;
                }
            }
            off = o;
        }
        int getInt(Object o) {
            if (off >= 0) return U.getInt(o, off);
            try {
                return vh != null ? (int) vh.get(o) : f.getInt(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        long getLong(Object o) {
            if (off >= 0) return U.getLong(o, off);
            try {
                return vh != null ? (long) vh.get(o) : f.getLong(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        void setInt(Object o, int v) {
            if (off >= 0) { U.putInt(o, off, v); return; }
            try {
                if (vh != null) vh.set(o, v); else f.setInt(o, v);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        Object getObj(Object o) {
            if (off >= 0) return U.getObject(o, off);
            try {
                return vh != null ? vh.get(o) : f.get(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        void setObj(Object o, Object v) {
            if (off >= 0) { U.putObject(o, off, v); return; }
            try {
                if (vh != null) vh.set(o, v); else f.set(o, v);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        // L36: primitive readers for the idle-control guards. getBool reads
        // the 1-byte boolean slot via Unsafe.getByte (a raw getInt would
        // straddle into the adjacent field — the L33 1-byte rule).
        boolean getBool(Object o) {
            if (off >= 0) return U.getByte(o, off) != 0;
            try {
                return vh != null ? (boolean) vh.get(o) : f.getBoolean(o);
            } catch (IllegalAccessException neverWithSetAccessible) {
                throw new IllegalStateException(neverWithSetAccessible);
            }
        }
        float getFloat(Object o) {
            if (off >= 0) return U.getFloat(o, off);
            try {
                return vh != null ? (float) vh.get(o) : f.getFloat(o);
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
            // L20b: the rest branch reads despawnRate every rest tick —
            // direct offset when available (plain long, published by the
            // volatile VH_READY below)
            O_DESPAWN_RATE = VH_DESPAWN_RATE.off;
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
        if (e.level().isClientSide()) return false; // client-side ticks stay vanilla
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
                && !e.level().isClientSide()) {
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
        IB_REF[i] = e;
        // L8.2: side-pack the collector-data stride into IB_CD, slot ci+1
        int ci = ibCDN;
        int o = ci * 14;
        if (IB_CD.length < o + 14) {
            double[] nd = new double[Math.max(o + 14, IB_CD.length * 2)];
            System.arraycopy(IB_CD, 0, nd, 0, o);
            IB_CD = nd;
        }
        Vec3 d = e.getDeltaMovement();
        IB_CD[o] = e.getX();
        IB_CD[o + 1] = e.getY();
        IB_CD[o + 2] = e.getZ();
        IB_CD[o + 3] = d.x;
        IB_CD[o + 4] = d.y;
        IB_CD[o + 5] = d.z;
        // L44: the PRE-body box capture (the vanilla per-body scan view).
        // The ci-th entry's 6 doubles ride in IB_PRE[ci*6 .. +5]; the flush
        // exposes them to the query paths as the VIEW for resync entries.
        int ci6 = ci * 6;
        if (IB_PRE.length < ci6 + 6) {
            double[] np = new double[Math.max(ci6 + 6, IB_PRE.length * 2)];
            System.arraycopy(IB_PRE, 0, np, 0, Math.min(IB_PRE.length, ci6));
            IB_PRE = np;
        }
        AABB pbox = e.getBoundingBox();
        IB_PRE[ci6] = pbox.minX;
        IB_PRE[ci6 + 1] = pbox.minY;
        IB_PRE[ci6 + 2] = pbox.minZ;
        IB_PRE[ci6 + 3] = pbox.maxX;
        IB_PRE[ci6 + 4] = pbox.maxY;
        IB_PRE[ci6 + 5] = pbox.maxZ;
        IB_CD[o + 6] = fallDistOf(e);
        // stuck + supporting are store-owned; only resync entries get fresh
        // values (flush pass 1)
        for (int j = 7; j < 14; j++) IB_CD[o + j] = 0.0;
        ibCDN = ci + 1;
        IB_FLAGS[i] |= (ci + 1) << IF_CDSHIFT;
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
        CUR_TICK++;
        if (n == 0) return;
        ibN = 0;
        int cdN = ibCDN;
        ibCDN = 0;
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
                int f = IB_FLAGS[i] & IF_CDMASK;
                if ((f & (IF_RESYNC | IF_FORCE_VANILLA)) != 0) {
                    Entity ref = IB_REF[i];
                    if (!(ref instanceof ItemEntity) || ref.isRemoved()) continue;
                    ItemEntity e = (ItemEntity) ref;
                    int o = ((IB_FLAGS[i] >>> IF_CDSHIFT) - 1) * 14; // cd slot
                    IB_CD[o] = e.getX();
                    IB_CD[o + 1] = e.getY();
                    IB_CD[o + 2] = e.getZ();
                    Vec3 d = e.getDeltaMovement();
                    IB_CD[o + 3] = d.x;
                    IB_CD[o + 4] = d.y;
                    IB_CD[o + 5] = d.z;
                    IB_CD[o + 6] = fallDistOf(e);
                    Vec3 stuck = (Vec3) VH_STUCK.getObj(e);
                    IB_CD[o + 7] = stuck == null ? 0.0 : stuck.x;
                    IB_CD[o + 8] = stuck == null ? 0.0 : stuck.y;
                    IB_CD[o + 9] = stuck == null ? 0.0 : stuck.z;
                    Object sup = e.mainSupportingBlockPos;
                    if (sup instanceof java.util.Optional<?> opt && opt.isPresent()
                            && opt.get() instanceof BlockPos bp) {
                        IB_CD[o + 10] = bp.getX();
                        IB_CD[o + 11] = bp.getY();
                        IB_CD[o + 12] = bp.getZ();
                        IB_CD[o + 13] = 1.0;
                    } else {
                        IB_CD[o + 13] = 0.0;
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

            int rc = nativeItemTick(wid, n, IB_IDS, IB_FLAGS, IB_CD, cdN, IB_I,
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
                int nSd = sdN;
                sdN = 0;
                sdWid = -1;
                WORLD_REV = revBefore + 1;
                // L21 rest: stamp the touched sections with the post-bump
                // revision — the rest branch exits only items whose tick
                // environment (box span + the below block) was touched
                for (int si = 0; si < nSd; si++) {
                    SECT_REV[sectKeySlot(SD_X[si], SD_Y[si], SD_Z[si])] = WORLD_REV;
                }
            }

            // L44: build the merge-scan view — the batch's RESYNC entries'
            // PRE-body boxes (their bodies ran during the phase; the live
            // boxes are already POST-move, one physics step past the vanilla
            // per-body scan view). VIEW_MAP: id -> view slot; VIEW_KIDX: the
            // entry's batch index (the vanilla body order).
            viewN = 0;
            VIEW_MAP.clear();
            for (int i = 0; i < n; i++) {
                int f = IB_FLAGS[i] & IF_CDMASK;
                // RESYNC and FORCE_VANILLA entries both ran their vanilla
                // bodies DURING the phase (their live boxes are POST-move);
                // inactive entries never move — no view needed for them.
                if ((f & (IF_RESYNC | IF_FORCE_VANILLA)) == 0
                        || (f & IF_INACTIVE) != 0) continue;
                if (viewN >= VIEW_IDS.length) break;
                Entity ref = IB_REF[i];
                if (!(ref instanceof ItemEntity) || ref.isRemoved()) continue;
                int cdSlot = (IB_FLAGS[i] >>> IF_CDSHIFT) - 1;
                if (cdSlot < 0) continue;
                VIEW_IDS[viewN] = (int) (IB_IDS[i] & 0xFFFF_FFFFL);
                int p6 = cdSlot * 6;
                if (IB_PRE.length >= p6 + 6) {
                    VIEW_BOX[viewN * 6] = IB_PRE[p6];
                    VIEW_BOX[viewN * 6 + 1] = IB_PRE[p6 + 1];
                    VIEW_BOX[viewN * 6 + 2] = IB_PRE[p6 + 2];
                    VIEW_BOX[viewN * 6 + 3] = IB_PRE[p6 + 3];
                    VIEW_BOX[viewN * 6 + 4] = IB_PRE[p6 + 4];
                    VIEW_BOX[viewN * 6 + 5] = IB_PRE[p6 + 5];
                } else {
                    AABB lb = ref.getBoundingBox();
                    VIEW_BOX[viewN * 6] = lb.minX;
                    VIEW_BOX[viewN * 6 + 1] = lb.minY;
                    VIEW_BOX[viewN * 6 + 2] = lb.minZ;
                    VIEW_BOX[viewN * 6 + 3] = lb.maxX;
                    VIEW_BOX[viewN * 6 + 4] = lb.maxY;
                    VIEW_BOX[viewN * 6 + 5] = lb.maxZ;
                }
                VIEW_KIDX[viewN] = i;
                VIEW_MAP.put(VIEW_IDS[viewN], viewN);
                viewN++;
            }

            // pass 2: apply, in batch order (== vanilla entity tick order).
            // The writebacks fire setPos/setDeltaMovement → probeBox/probeDelta
            // — IN_ITEM_APPLY marks them as OUR writes (no mode clearing).
            IN_ITEM_APPLY = true;
            try {
            int epoch = MODE_EPOCH + 1;
            for (int i = 0; i < n; i++) {
                int idSlot = (int) (IB_IDS[i] & 0xFFFF_FFFF);
                int f = IB_FLAGS[i] & IF_CDMASK;
                int bits = outI[i * 4 + 2];
                int mSlot = (int) (IB_IDS[i] & MAMASK);
                MODE_GEN[mSlot] = epoch;
                if ((bits & (1 << 11)) != 0) { // VANILLA_NEXT
                    MODE_REV[mSlot] = revBefore;
                    MODE_VAL[mSlot] = 0;
                    itemVanillaNext++;
                } else if ((bits & (1 << 12)) != 0) { // NATIVE_OK
                    // L21 REST: a value-fixed-point tick — the dispatch guard
                    // takes over the counter arithmetic and the item leaves
                    // the flush cycle until something wakes it (probe delta/
                    // box, section change, merge cadence, the 256-tick belt,
                    // despawn). MODE_REV is stamped with the POST-drain
                    // revision so a stale-but-boring world revision does not
                    // herd every rest item vanilla.
                    //
                    // L37f REST DISABLED (parity): the fixed point is REAL on
                    // %4 gate-open ticks (the collision zeroes the gravity —
                    // dy 0.0 -> 0.0 — the trace-proven rest arm at
                    // items-T389), but the inline guard then runs COUNTERS
                    // ONLY: it misses the every-%4 gravity+move+friction
                    // cycle and the 1-in-20 activation windows that vanilla
                    // still applies to a rested item (Paper 26.3
                    // checkIfActive's ItemEntity %4 exception keeps items
                    // full-ticking forever). Until the inline path mirrors
                    // the full per-tick semantics, rest stays off: items
                    // stay in the native batch where tick_item is exact.
                    Entity r0 = IB_REF[i];
                    if (false
                            && (bits & (1 << 15)) != 0
                            && r0 instanceof ItemEntity ie0
                            && !restSectionTouched(ie0)) {
                        MODE_VAL[mSlot] = 2;
                        REST_ID[mSlot] = IB_IDS[i];
                        MODE_REV[mSlot] = WORLD_REV;
                    } else {
                        MODE_REV[mSlot] = revBefore;
                        MODE_VAL[mSlot] = 1;
                    }
                } else {
                    MODE_REV[mSlot] = revBefore;
                    MODE_VAL[mSlot] = 0; // resync/vanilla-ran re-enters via
                                         // want_native next flush
                }
                if ((f & IF_NATIVE) == 0 || (bits & (1 << 13)) != 0) {
                    // aborted (bit 13): the core did not compute — nothing to
                    // apply, the item froze this tick and runs vanilla next.
                    if ((bits & (1 << 13)) != 0) itemAbort++;
                    continue;
                }
                Entity ref = IB_REF[i];
                if (!(ref instanceof ItemEntity) || ref.isRemoved()) continue;
                ItemEntity e = (ItemEntity) ref;
                // L7: mirror the suppressed dispatcher's tickCount++ and
                // totalEntityAge++ (the vanilla tickNonPassenger body was
                // skipped for dispatch-owned entries).
                e.tickCount = outI[i * 4 + 3];
                e.totalEntityAge = e.totalEntityAge + 1;
                if ((f & IF_INACTIVE) != 0) {
                    // inactiveTick semantics: pickupDelay/age/despawn only
                    e.age = outI[i * 4];
                    int pd = outI[i * 4 + 1];
                    if (pd != IB_I[i * 4 + 2]) { // same-value skip (resting items)
                        e.pickupDelay = pd;
                    }
                    if ((bits & (1 << 7)) != 0) {
                        int rate = VH_DESPAWN_RATE.getInt(e);
                        // IF_NATIVE entries carry no batch despawnRate — the
                        // field IS the truth (ctor-fixed in vanilla); the
                        // age check below still gates the event.
                        boolean rateOk = (f & IF_NATIVE) != 0
                                || rate == IB_I[i * 4 + 3];
                        if (rateOk && e.age >= rate) {
                            despawnTail(e);
                        }
                    }
                    continue;
                }
                int o = i * 8;
                // L16 DELTA_SAME: when the core reports the post-tick delta
                // is bit-identical to the pre-tick delta (the resting-item
                // fixed point: gravity -0.04, collide zeroes the axis,
                // 0.0 * friction == 0.0), vanilla would still allocate a
                // fresh Vec3 and rewrite the field — the FIELD VALUE is the
                // same, so skipping the write is value-exact. This drops
                // ~100k Vec3 allocations + virtual writes per tick on
                // items-100k (setDeltaMovement self was 3.5% of the leg).
                if ((bits & (1 << 14)) == 0) { // not DELTA_SAME
                    e.setDeltaMovement(new Vec3(outD[o], outD[o + 1], outD[o + 2]));
                }
                e.age = outI[i * 4];
                int pd = outI[i * 4 + 1];
                if (pd != IB_I[i * 4 + 2]) { // same-value skip (resting items)
                    e.pickupDelay = pd;
                }
                float fd = (float) outD[o + 6];
                if (fd != e.fallDistance) { // same-value skip (resting items)
                    setFallDist(e, fd);
                }
                boolean wantGround = (bits & (1 << 1)) != 0;
                if (wantGround != e.onGround()) {
                    e.onGround = wantGround;
                }
                if ((bits & 1) != 0) { // MOVED
                    e.setPos(outD[o + 3], outD[o + 4], outD[o + 5]);
                }
                if ((bits & (1 << 2)) != 0) { // HAS_IMPULSE (OR-only)
                    // 26.3: Entity.hasImpulse was renamed needsSync
                    // (same role: forces the tracker to sync the delta)
                    e.needsSync = true;
                }
                if ((bits & (1 << 5)) != 0) { // MERGE cadence -> exact L3 path
                    itemMerge++;
                    // MS diagnostics: the flush path decided this scan; mv
                    // carries the Rust cadence's floor-change verdict (rate 2
                    // vs 40) — the mixed t=152 / items t=159 analysis wants
                    // the cadence split per scanner, not just the scan list.
                    MS_SRC = 1;
                    MS_MV = (bits & (1 << 6)) != 0 ? 1 : 2;
                    // L44: this scanner's batch index = its vanilla body
                    // position; the view entries after it are seen PRE-move.
                    MS_KIDX = i;
                    // A refusal (index unsure) or a disabled fast path
                    // (CE_MERGE=0 knob) MUST fall back to the vanilla body —
                    // an ignored false here DROPPED the merge outright
                    // (observed: 104,383 dropped merges in one spread leg).
                    // State is fresh: age/pickupDelay/pos are already applied
                    // above in vanilla tick order, so mergeWithNeighbours
                    // sees exactly what vanilla's mid-tick body would see.
                    if (!mergeScanNative(e)) {
                        runVanillaMerge(e);
                    }
                    if (e.isRemoved()) continue;
                }
                if ((bits & (1 << 7)) != 0) { // DESPAWN: exact vanilla snippet
                    int rate = VH_DESPAWN_RATE.getInt(e);
                    boolean rateOk = (f & IF_NATIVE) != 0
                            || rate == IB_I[i * 4 + 3];
                    if (rateOk && e.age >= rate) {
                        despawnTail(e);
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

    /** The exact vanilla despawn tail (ItemEntity.tick/inactiveTick):
     *  callItemDespawnEvent -> cancelled ? age = 0 : discard(Cause.DESPAWN).
     *  Shared by the flush apply and the rest branch's inline arithmetic. */
    private static void despawnTail(ItemEntity e) throws Throwable {
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
        // AIR entry — the offsets MUST match writeEntry/Rust parse_section
        // (PAL_STRIDE=24: 0=collider-count, 1-18=up to 3 cuboids, 19=friction,
        // 20=speed, 21=water, 22=fluid, 23=flags).
        // BUG (found via the 100k-fill RSS explosion, ~457k cached sections):
        // friction/speed were written at 8/9 and water was left at 1.0 — the
        // parser read air as {friction 0.0, speed 0.6, IS_WATER=1.0}, so every
        // section reported any_fluid and the core's fluid path ran on garbage.
        outD[0] = 0.0; // air entry: no colliders
        outD[19] = 0.6; // friction
        outD[20] = 1.0; // speed factor
        outD[21] = 0.0; // water: NO
        outD[22] = 0.0; // fluid: NO
        outD[23] = 0.0; // flags: none
        entryN = 1;
        try {
            net.minecraft.world.level.block.state.BlockState[] states =
                    new net.minecraft.world.level.block.state.BlockState[4096];
            int nn = 0;
            for (int y = 0; y < 16; y++) {
                for (int z = 0; z < 16; z++) {
                    for (int x = 0; x < 16; x++) {
                        int local = x | (z << 4) | (y << 8);
                        BlockState st = section.getStates().get(x, y, z);
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
            if (COMPLEX_DIAG.tryAcquire()) {
                for (java.util.Map.Entry<BlockState, Integer> en : paletteIdx.entrySet()) {
                    var be = en.getKey().getBlock();
                    var bs = en.getKey().getCollisionShape(
                            net.minecraft.world.level.EmptyBlockGetter.INSTANCE,
                            BlockPos.ZERO, net.minecraft.world.phys.shapes.CollisionContext.empty())
                            .toAabbs().size();
                    // L8.2 (doc 19 lever 4): the zero-cuboid shape-overrides
                    // (air, plants, torches) are now HANDLED natively (complex
                    // stays 0 for them) — logging every such entry poisoned the
                    // diag with noise. Restrict to genuinely-complex entries:
                    // >3 cuboids or a shape-override that still carries boxes.
                    if (bs > 3 || (overridesShape(en.getKey()) && bs > 0)) {
                        System.out.println("[c-entities] section palette complex entry: "
                                + be.getClass().getName() + " (" + be + ") boxes=" + bs
                                + " slot=" + en.getValue());
                    }
                }
            }
        } catch (Throwable t) {
            return false;
        }
        return true;
    }

    private static final java.util.concurrent.Semaphore COMPLEX_DIAG =
            new java.util.concurrent.Semaphore(4);

    /** One palette entry -> 18 packed doubles at slot * 18. */
    /** Palette blob stride: 24 doubles per entry. Layout per slot:
     *  0=collider-count (0..3), 1-6=cuboid0, 7-12=cuboid1, 13-18=cuboid2
     *  (unused slots stay 0), 19=friction, 20=speed, 21=water, 22=fluid,
     *  23=flags packed in the low bits: cobweb=1, entityInside=2, stepOn=4,
     *  fallOn=8, shapeOverride=16, complex=32. */
    static final int PAL_STRIDE = 24;

    private static void writeEntry(double[] outD, int slot, BlockState st) {
        int o = slot * PAL_STRIDE;
        var shape = st.getCollisionShape(
                net.minecraft.world.level.EmptyBlockGetter.INSTANCE,
                BlockPos.ZERO, net.minecraft.world.phys.shapes.CollisionContext.empty());
        var boxes = shape.toAabbs();
        boolean shapeOv = overridesShape(st);
        // context-dependent shapes with actual cuboids (walls, fences, ...)
        // still route vanilla; ZERO-cuboid shape-overrides (air, plants,
        // torches — empty collision everywhere) are natively exact as 0
        // cuboids and must NOT poison the whole section.
        int complex = (shapeOv && !boxes.isEmpty()) ? 32 : 0;
        int n = 0;
        if (!shapeOv || boxes.isEmpty()) {
            // genuine multi-cuboid shapes (stairs, beds, ...) are carried
            // natively: up to 3 cuboids per entry, exact collide.
            for (int i = 0; i < boxes.size() && n < 3; i++) {
                var b = boxes.get(i);
                outD[o + 1 + n * 6] = b.minX;
                outD[o + 2 + n * 6] = b.minY;
                outD[o + 3 + n * 6] = b.minZ;
                outD[o + 4 + n * 6] = b.maxX;
                outD[o + 5 + n * 6] = b.maxY;
                outD[o + 6 + n * 6] = b.maxZ;
                n++;
            }
            if (boxes.size() > 3) complex = 32; // >3 cuboids: rare; route vanilla
        }
        outD[o] = n;
        outD[o + 23] = (st.is(net.minecraft.world.level.block.Blocks.COBWEB) ? 1 : 0)
                | (overrides(st, "entityInside") ? 2 : 0)
                | (overrides(st, "stepOn") ? 4 : 0)
                | (overrides(st, "bounceOn") ? 8 : 0) // 26.3: was updateEntityMovementAfterFallOn
                | complex;
        outD[o + 19] = st.getBlock().getFriction();
        outD[o + 20] = st.getBlock().getSpeedFactor();
        outD[o + 21] = (st.is(net.minecraft.world.level.block.Blocks.WATER)
                || st.is(net.minecraft.world.level.block.Blocks.BUBBLE_COLUMN)) ? 1.0 : 0.0;
        outD[o + 22] = st.getFluidState().isEmpty() ? 0.0 : 1.0;
    }

    /** true when the block class overrides the (possibly renamed) method.
     *
     *  1.21.8 ROOT-CAUSE FIX (the ~48% abort storm): stepOn / fallOn /
     *  updateEntityMovementAfterFallOn MOVED from BlockBehaviour into the
     *  Block base class itself (javap-verified on the shipped 1.21.8-60
     *  jar). Anchoring the walk at BlockBehaviour therefore matched EVERY
     *  block in the game — fall_on_special=1 everywhere — and the tick
     *  core's swept-gather gate aborted every item whose sweep touched any
     *  block. The anchor is now Block (the defaults' new home), so only
     *  genuine overriders below Block (slime, honey, cobweb, ...) flag.
     *
     *  26.3 RETARGET: updateEntityMovementAfterFallOn was renamed to
     *  bounceOn(Level, BlockState, BlockPos, Entity, double) by Mojang
     *  (fall-damage rework; javap-verified on the shipped 26.3-218 jar —
     *  Entity calls Block.bounceOn where 1.21.8 called
     *  updateEntityMovementAfterFallOn). Same anchor semantics. */
    private static boolean overrides(BlockState st, String name) {
        try {
            Class<?> k = st.getBlock().getClass();
            while (k != null && k != net.minecraft.world.level.block.Block.class) {
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
            double[] cd, int cdN, int[] inI, double[] envD, int[] envI,
            double[] outD, int[] outI);
    public static native void nativeItemInvalidate(
            int wid, int n, int[] xs, int[] ys, int[] zs);

    // ===================================================================
    // L14 — the tracker fast gate (ServerEntity.sendChanges)
    //
    // javap-verified against the shipped 26.3 build (doc 25): on the path
    // where NOTHING is sent, sendChanges performs exactly three observable
    // effects —
    //   1. entity.updateDataBeforeSync()   (virtual; LivingEntity adds
    //      updateDirtyEffects)
    //   2. interpolationTracker.updateTracking(trackingPosition()) — the
    //      NO_OP tracker's updateTracking is an empty body, so skipping the
    //      Vec3 allocation for NO_OP entities is exact
    //   3. tickCount++                     (the unconditional tail, bc 1106)
    // Everything else is gated behind conditions the guard re-evaluates in
    // the vanilla order (bc 347-390): hardColliding → vanilla (teleportDelay
    // bookkeeping); syncPosition → vanilla (interval reset side effect);
    // needsSync / syncVelocity / forceStateResync → vanilla; then the pure
    // gate reads: updateInterval.test (PURE: n % period == 0, javap'd both
    // implementations), SynchedEntityData.isDirty (a flag read). The
    // passenger compare (bc 34-44) routes vanilla whenever it differs —
    // vanilla writes lastPassengers and emits the packet there. ItemFrame
    // map ticks and all emit paths live behind trackedPlayers.isEmpty(),
    // which the guard requires.
    //
    // FALLBACK CONTRACT: any surprise routes vanilla (return false — the
    // vanilla body is intact behind the early-return weave). A THROW inside
    // the guard would propagate into the tick loop and kill the server, so
    // the catch is mandatory, not cosmetic.
    // L15b: bumped per sendChanges (100k/tick on items-100k) — PLAIN, same
    // rationale as mergeChecks above (verifier/e2e tolerate eventual visibility).
    public static long trackerChecks = 0;   // guard calls (weave proof)
    public static volatile long trackerFast = 0;     // provable no-ops served
    public static volatile long trackerVanilla = 0;  // routed to the body
    public static volatile long trackerErr = 0;      // unexpected → vanilla

    /** L14 kill switch: -Dcentities.trackerFast=false keeps the vanilla
     *  sendChanges body for every entity. */
    public static final boolean TRACKER_ON =
            !"false".equals(System.getProperty("centities.trackerFast", "true"));

    // ServerEntity privates (the guard runs from a different package).
    private static volatile RefField VH_SE_ENTITY, VH_SE_TICK_COUNT,
            VH_SE_LAST_PAX, VH_SE_TRACKED_PLAYERS, VH_SE_INTERP,
            VH_SE_INTERVAL, VH_SE_RESYNC;
    private static volatile boolean SE_VH_FAILED = false;

    private static synchronized boolean initTrackerVh() {
        if (SE_VH_FAILED) return false;
        if (VH_SE_TICK_COUNT != null) return true;
        // ATOMIC: build all seven into locals; publish only on complete
        // success (the L12 lesson: a half-published handle set NPEs mid-tick)
        try {
            Class<?> se = Class.forName("net.minecraft.server.level.ServerEntity");
            RefField en = new RefField(se, "entity");
            RefField tc = new RefField(se, "tickCount");
            RefField lp = new RefField(se, "lastPassengers");
            RefField tp = new RefField(se, "trackedPlayers");
            RefField it = new RefField(se, "interpolationTracker");
            RefField iv = new RefField(se, "updateInterval");
            RefField rs = new RefField(se, "forceStateResync");
            VH_SE_ENTITY = en;
            VH_SE_TICK_COUNT = tc;
            VH_SE_LAST_PAX = lp;
            VH_SE_TRACKED_PLAYERS = tp;
            VH_SE_INTERP = it;
            VH_SE_INTERVAL = iv;
            VH_SE_RESYNC = rs;
            // L26: direct offsets for the fast-exit reads. DIFFERENT from the
            // retracted L19/20b path: the doc-32 rule is applied AT ENTRY —
            // the weave guarantees `se` is ServerEntity (the replaced body's
            // own receiver), and every offset here is single-class
            // ServerEntity. The old crash fed a FOREIGN receiver (a
            // TrackedEntity) into ServerEntity offsets without a check.
            if (SE_DIRECT_ON) {
                O_SE_ENTITY = en.off;
                O_SE_TICKCOUNT = tc.off;
                O_SE_TRACKED_PLAYERS = tp.off;
                O_SE_INTERP = it.off;
            }
            return true;
        } catch (Throwable t) {
            SE_VH_FAILED = true;
            lastError = System.nanoTime();
            lastErrorText = "sevh: " + t;
            return false;
        }
    }

    // L19/20b: the direct-offset (Unsafe raw offsets, plain static longs)
    // fast path was RETRACTED at 7c88ee2 and is now DELETED (L24 verdict,
    // 2026-10-25): the hs_err pair 4417+5536 was decoded offline — both are
    // G1SATBMarkQueueSet::filter SIGSEGVs whose queue held a native-range
    // garbage oop; the crash spot (trackerDispatch's own barrier) is just
    // where the per-thread buffer FILLED (trackerDispatch is the 100k/tick
    // store), not necessarily the corruption site. The run correlation is
    // conclusive: every L22-era crash ran with the direct-offset path ON,
    // every green run before AND after ran with it OFF (67% crash rate with,
    // 0% without, same agent, same legs). The mechanism stays unresolved
    // (the module's own Rust/JNI surface was audited clean — no raw writes,
    // no cached refs, bounds-matched buffers), so the path stays out; the
    // VH path below is the proven form and the L24 mark-bitmap gate
    // replaces its speed in a barrier-safe way.

    public static boolean trackerDispatch(ServerEntity se) {
        // L15b: volatile increment per entity per tick = 100k lock-xadds/tick
        // on items-100k (~1-2ms) — plain like the other hot-path counters
        // (the JNI verifier tolerates staleness).
        trackerChecks++;
        if (!ACTIVE || !TRACKER_ON || !cplugReady) return false;
        if (VH_SE_ENTITY == null && !initTrackerVh()) return false;
        try {
            // L26: the two fast-exit reads go through single-class direct
            // offsets when SE_DIRECT_ON (typed ServerEntity receiver — the
            // weave-guaranteed `this` of the replaced sendChanges body).
            final Entity e;
            final Object trk;
            final Object interp;
            if (SE_DIRECT_ON && O_SE_ENTITY >= 0) {
                e = (Entity) U.getObject(se, O_SE_ENTITY);
                trk = U.getObject(se, O_SE_TRACKED_PLAYERS);
                interp = U.getObject(se, O_SE_INTERP);
            } else {
                e = (Entity) VH_SE_ENTITY.getObj(se);
                trk = VH_SE_TRACKED_PLAYERS.getObj(se);
                interp = VH_SE_INTERP.getObj(se);
            }
            // L24a — the empty-tracking fast exit. javap 26.3 (doc 31): with
            // trackedPlayers EMPTY the vanilla sendChanges body cannot send
            // anything — every emit rides sendToTrackingPlayers(AndSelf)
            // over an empty set. What remains: updateDataBeforeSync (we
            // reproduce), tickCount++ (we reproduce), the syncPosition /
            // needsSync / forceStateResync / isDirty "changed" latches and
            // the lastPassengers state compare — all of which vanilla leaves
            // set/stale and the FIRST sendChanges that actually has a
            // recipient consumes (every one of them bypasses the interval
            // gate, so a latched flag self-heals with an exact vanilla emit
            // the moment a player pairs). The fast exit keeps three
            // conditions that route vanilla exactly where the skipped tail
            // would have had a real effect.
            if (trk instanceof java.util.Set<?> ts && ts.isEmpty()
                    // a non-NO_OP interpolation tracker's updateTracking
                    // advances real state every tick; a hard-colliding
                    // entity gets teleportDelay=9999 written every tick
                    // (bc 0-16); syncVelocity entities hit the tail's
                    // velocity branch which consumes the flag + fires the
                    // PlayerVelocityEvent path for ServerPlayers
                    // (bc 1116-1246). Any hit → the full gate below (the
                    // exact vanilla mirror).
                    && interp == net.minecraft.world.entity.InterpolationTracker.NO_OP
                    && !e.moonrise$isHardColliding()
                    && !e.syncVelocity) {
                e.updateDataBeforeSync();                       // bc 19-23
                if (SE_DIRECT_ON && O_SE_TICKCOUNT >= 0) {
                    U.putInt(se, O_SE_TICKCOUNT, U.getInt(se, O_SE_TICKCOUNT) + 1);
                } else {
                    VH_SE_TICK_COUNT.setInt(se,
                            VH_SE_TICK_COUNT.getInt(se) + 1);   // bc 1106-1113
                }
                trackerFast++;
                return true;
            }
            // tracked entities and fast-exit rejects: the full gate below
            // decides — one extra tracked-read total for tracked entities.
            // L15b REORDER: every read below is PURE (no side effects — the
            // two unconditional effects ride at the tail in vanilla order),
            // so the ORDER of the reads is free, and a throw from any read
            // propagates with zero state changed either way. This full gate
            // now serves only the entities that HAVE trackers (or hit a fast
            // exit reject): the exact vanilla-order mirror.
            if (e.getEntityData().isDirty()) return false;      // bc 384-390
            if (e.syncPosition || e.syncVelocity || e.needsSync) return false; // bc 312-346
            // bc 0-16: hard-colliding entities own the teleportDelay path
            if (e.moonrise$isHardColliding()) return false;
            // bc 302-311: the snapshot only matters for real trackers; the
            // NO_OP implementation (javap'd) is an empty body — skipping its
            // trackingPosition() Vec3 allocation is exact
            if (VH_SE_INTERP.getObj(se)
                    != net.minecraft.world.entity.InterpolationTracker.NO_OP) {
                return false;
            }
            // bc 34-44: passengers changed → vanilla (writes lastPassengers,
            // emits ClientboundSetPassengersPacket)
            Object pax = e.getPassengers();
            if (!pax.equals(VH_SE_LAST_PAX.getObj(se))) return false;
            // bc 107-116: non-empty tracking → ItemFrame map tick + emit paths
            java.util.Set<?> tracked =
                    (java.util.Set<?>) VH_SE_TRACKED_PLAYERS.getObj(se);
            if (tracked == null || !tracked.isEmpty()) return false;
            // bc 347: forceStateResync → the full resync body
            if (VH_SE_RESYNC.getObj(se) == Boolean.TRUE) return false;
            // bc 364-377: updateInterval.test is PURE (n % period == 0)
            net.minecraft.world.entity.UpdateInterval interval =
                    (net.minecraft.world.entity.UpdateInterval) VH_SE_INTERVAL.getObj(se);
            int tickCount = VH_SE_TICK_COUNT.getInt(se);
            if (interval == null || interval.test(tickCount)) return false;
            // ALL conditions passed: this tick emits nothing. Reproduce the
            // unconditional effects in vanilla order:
            e.updateDataBeforeSync();                       // bc 19-23
            VH_SE_TICK_COUNT.setInt(se, tickCount + 1);     // bc 1106-1113
            trackerFast++;
            return true;
        } catch (Throwable t) {
            // A stale RefField (class reload) is the only realistic failure:
            // rebuild once against the live class object, then give up —
            // vanilla is the only safe direction.
            trackerErr++;
            lastError = System.nanoTime();
            VH_SE_TICK_COUNT = null; // forces initTrackerVh() to rebuild
            initTrackerVh();
            return false;
        }
    }

    // ===================================================================
    // L13 — the tick-list raw walk (EntityTickList.forEach)
    //
    // javap-verified against 26.3 (doc 25): vanilla forEach = iterator()
    // (BaseIterator, canFinish=true, maxIndex = the listSize SNAPSHOT at
    // creation) + hasNext/next + accept + finishedIterating in a
    // try/finally. The mirror reproduces the exact visit sequence with the
    // PUBLIC raw API of the moonrise set:
    //   - visit order = array order (insertion), nulls skipped — remove()
    //     nulls the slot immediately (bc 51-57), even mid-iteration
    //   - entities ADDED during the walk are NOT visited (the snapshot cap;
    //     BaseIterator caps at min(maxIndex, listSize) the same way)
    //   - iteratorCount++/-- and the defrag-on-finish bookkeeping ride on
    //     createRawIterator()/finishRawIterator() — same counters, same
    //     defrag condition (iteratorCount==0 → frag-factor check)
    //   - a consumer exception still passes finishRawIterator (try/finally)
    //     before propagating, exactly like vanilla's exception table
    // This path is behaviorally exact with the module ARMED or DISARMED
    // (it only changes the iteration strategy), so it does not read ACTIVE:
    // both A/B legs walk the same way and the gate stays honest.
    // L15b: PLAIN (per level tick — cheap, but the read cadence is the same
    // eventual-visibility contract).
    public static long tickListChecks = 0;  // mirror calls (weave proof)
    public static volatile long tickListErr = 0;     // retries served

    /** L13 kill switch: -Dcentities.tickListRaw=false refuses the weave at
     *  precompute time (vanilla forEach stays). */
    public static final boolean TICKLIST_ON =
            !"false".equals(System.getProperty("centities.tickListRaw", "true"));

    private static volatile RefField VH_TL_ENTITIES;
    private static volatile boolean TL_VH_FAILED = false;

    // L26: direct ServerEntity offsets for the trackerDispatch fast exit
    // (single-class, typed receiver — see the comment at the capture site).
    // SE_DIRECT_ON is its own kill switch so a bisect can cut exactly this
    // path without losing the L24a fast exit.
    public static final boolean SE_DIRECT_ON =
            !"false".equals(System.getProperty("centities.trackerDirect", "true"));
    private static long O_SE_ENTITY = -1, O_SE_TICKCOUNT = -1,
            O_SE_TRACKED_PLAYERS = -1, O_SE_INTERP = -1;

    private static synchronized boolean initTickListVh() {
        if (TL_VH_FAILED) return false;
        if (VH_TL_ENTITIES != null) return true;
        try {
            VH_TL_ENTITIES = new RefField(
                    Class.forName("net.minecraft.world.level.entity.EntityTickList"),
                    "entities");
            // SELF-TEST: prove the handle actually READS before the weave is
            // installed — the replaced forEach body has no vanilla escape, so
            // a handle that builds but cannot read would crash the first tick.
            Object probe = new net.minecraft.world.level.entity.EntityTickList();
            VH_TL_ENTITIES.getObj(probe);
            return true;
        } catch (Throwable t) {
            TL_VH_FAILED = true;
            lastError = System.nanoTime();
            lastErrorText = "tlvh: " + t;
            return false;
        }
    }

    /** Precompute-time probe result (read via GetStaticBooleanField — the
     *  variadic boolean-method slot is unusable from the Rust bindings). */
    public static volatile boolean TL_HANDLES_OK = false;

    /** Precompute-time probe: the private-handle set must be PROVEN before
     *  the forEach body is replaced (doc 25). Refusal keeps vanilla. */
    public static void initTickListHandles() {
        TL_HANDLES_OK = initTickListVh();
    }

    @SuppressWarnings("unchecked")
    public static void tickListForEach(net.minecraft.world.level.entity.EntityTickList tl,
                                       java.util.function.Consumer<Entity> consumer) {
        tickListChecks++;
        try {
            ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet<Entity> set =
                (ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet<Entity>)
                    VH_TL_ENTITIES.getObj(tl);
            // getListRaw() erases to Object[] (the no-arg ctor allocates
            // Object[]; the E[] signature is an unchecked moonrise-side cast —
            // assigning it to Entity[] threw ClassCastException on the first
            // world tick). Walk it as Object[] with an instanceof filter —
            // vanilla's BaseIterator does the same checkcast after next().
            Object[] arr = set.getListRaw();
            int maxIndex = set.getListSize();   // the snapshot semantics
            set.createRawIterator();            // iteratorCount++ (+ empty probe)
            try {
                // L17 collect-at-walk: the tick phase is the ONLY
                // EntityTickList.forEach caller (javap-verified, ServerLevel.tick),
                // so the walk can route native items straight into the flush
                // batch — skipping the guardEntityTick frame, the
                // tickNonPassenger frame and the accept dispatch for every
                // dispatch-owned item. The routing decision is the UNCHANGED
                // itemDispatch chain (mode table + fresh-flag belt + activation),
                // and a false keeps the vanilla accept path exactly as before.
                boolean collect = ACTIVE && ITEM_TICK_ON && cplugReady;
                for (int i = 0; i < maxIndex; i++) {
                    Object o = arr[i];
                    if (o instanceof Entity e) {
                        if (collect && e instanceof ItemEntity
                                && itemDispatch((ItemEntity) e)) {
                            continue; // native flush computes this tick
                        }
                        consumer.accept(e);
                    }
                }
            } finally {
                set.finishRawIterator();        // iteratorCount-- + defrag check
            }
        } catch (Throwable t) {
            // A stale RefField (class reload) is the only realistic failure.
            // Rebuild once against the live class and retry; a second failure
            // rethrows — silently skipping the entity tick phase is the one
            // outcome this weave must never produce.
            tickListErr++;
            lastError = System.nanoTime();
            lastErrorText = "tlwalk: " + t;
            VH_TL_ENTITIES = null;
            TL_VH_FAILED = false; // the retry deserves a fresh build attempt
            if (initTickListVh()) {
                try {
                    ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet<Entity> set =
                        (ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet<Entity>)
                            VH_TL_ENTITIES.getObj(tl);
                    Object[] arr = set.getListRaw();
                    int maxIndex = set.getListSize();
                    set.createRawIterator();
                    try {
                        boolean collect = ACTIVE && ITEM_TICK_ON && cplugReady;
                        for (int i = 0; i < maxIndex; i++) {
                            Object o = arr[i];
                            if (o instanceof Entity e) {
                                if (collect && e instanceof ItemEntity
                                        && itemDispatch((ItemEntity) e)) {
                                    continue;
                                }
                                consumer.accept(e);
                            }
                        }
                    } finally {
                        set.finishRawIterator();
                    }
                    return;
                } catch (Throwable fatal) {
                    // fall through to the honest rethrow
                }
            }
            if (t instanceof RuntimeException re) throw re;
            if (t instanceof Error err) throw err;
            throw new IllegalStateException("tickListForEach mirror failed", t);
        }
    }

    // ==================================================================
    // L15 — the AI floor: GoalSelector microkernel (doc 26).
    //
    // Profile of the 26.3 mobs-25k-mod leg: the vanilla scheduler burns
    // ~4.2% of self CPU across GoalSelector.tick (1.33) + EnumMap.get
    // (1.12, the lockedFlags getOrDefault chains) + tickRunningGoals
    // (0.72) + AbstractCollection.containsAll (0.70, the removeIf path)
    // + WrappedGoal.getFlags (0.37) — plus the callee side of every
    // EnumSet-sized lambda the map churn creates. The floor rewrites the
    // scheduler bookkeeping as flat long-bitmask arithmetic over a
    // per-thread lock table and a running-goal shadow, while calling the
    // goal objects in the EXACT vanilla order:
    //
    //  * same live-set iteration (ObjectLinkedOpenHashSet insertion order),
    //    same conditions, same short-circuit order — canUse/canContinueToUse
    //    are called iff vanilla calls them;
    //  * the lock table is REBUILT each cleanup from the running goals'
    //    current flags — exact for vanilla goals (javap-verified: every
    //    setFlags call in the shipped goal classes sits in a constructor,
    //    never in tick/canUse/start/stop). A table collision (two running
    //    goals holding one flag — impossible under vanilla semantics)
    //    disarms the floor globally and falls back to the mirror;
    //  * lockedFlags is private with no readers outside GoalSelector
    //    (decompile-verified) — the mirror reproduces it via RefField;
    //  * tickRunningGoals(false) walks the RUNNING shadow (insertion
    //    order, built each even tick) with a live isRunning re-check and
    //    lazy pruning, so a goal stopped between ticks is skipped exactly
    //    as vanilla skips it; the force=true tail walks the full set
    //    (identical to vanilla's tickRunningGoals(true));
    //  * profiler sections (goalCleanup/goalUpdate/goalTick) are kept —
    //    /debug tracer parity;
    //  * the floor is armed only after the replace_body weave is verified
    //    live (counters move), the handles are proven at precompute time,
    //    and -Dcentities.aiFloor=false keeps vanilla forever.
    // ==================================================================

    public static volatile boolean AI_ON = false;
    public static volatile boolean AI_HANDLES_OK = false;
    // per-tick counters are PLAIN (server-thread writes; the JNI verifier
    // reads tolerate staleness) — a volatile add per selector per tick would
    // burn ~0.3ms of the ~3ms the floor saves.
    public static long gsFloorTicks = 0, gsFloorRunning = 0, gsFloorStarts = 0,
            gsFloorStops = 0, gsFloorCollide = 0, gsMirrorTicks = 0;

    private static volatile RefField VH_GS_DISABLED; // GoalSelector.goalTypes
    private static volatile RefField VH_GS_LOCKED;   // GoalSelector.lockedFlags (mirror only)
    private static volatile int FLAG_N = -1;
    /** Cached Flag.values() — the mirror's EnumMap keys (ordinals map 1:1).
     *  Lazy: a static-init reference would force-load Goal during the module
     *  arm (a bootstrap-time class-load side effect we refuse to take). */
    private static volatile net.minecraft.world.entity.ai.goal.Goal.Flag[] GOAL_FLAGS;

    private static net.minecraft.world.entity.ai.goal.Goal.Flag[] goalFlags() {
        net.minecraft.world.entity.ai.goal.Goal.Flag[] f = GOAL_FLAGS;
        if (f == null) {
            f = net.minecraft.world.entity.ai.goal.Goal.Flag.values();
            GOAL_FLAGS = f;
        }
        return f;
    }

    /** Running-goal shadow: the running subset of availableGoals in the
     *  set's insertion order, rebuilt by every floor tick() pass. Held in
     *  a weak-keys identity map — an entry dies with its selector, and the
     *  per-call lookup (~20ns) replaces the impossible alternative (adding
     *  a field to a class the JVM already loaded: retransform forbids
     *  schema changes). */
    private static final class AIShadow {
        net.minecraft.world.entity.ai.goal.WrappedGoal[] goals =
                new net.minecraft.world.entity.ai.goal.WrappedGoal[8];
        int n;
        // L25 — the flat-scheduler shape stamp. allSize/first/last pin the
        // availableGoals identity the running[] snapshot was built from;
        // first()/last() are two direct field reads on fastutil's linked
        // set (javap-verified), so the per-tick validation costs three
        // compares. full=true (fresh shadow / post-mutation) forces the
        // exact 4-pass rebuild.
        int allSize = -1;
        Object first;
        Object last;
        boolean full = true;
    }

    // L16: the shadow map was a Guava MapMaker weak-keys ConcurrentMap —
    // the 26.3 mobs-25k-mod profile billed its segment chain
    // (Segment.getFirst 1.46% + getEntry 0.57%) per tick per selector
    // (25k mobs). This table is a flat open-addressed identity probe with
    // WEAK keys: one identityHashCode, one slot, one WeakReference.get()
    // — ~5-10ns. Dead entries (GC-cleared selectors) are reused in place
    // on the next probe that touches them; a periodic sweep rebuilds the
    // table when dead slots pile up (mob churn). Server-thread confined
    // (every caller runs inside the goal tick phase) — plain arrays, no
    // fences.
    private static final class AIKey extends java.lang.ref.WeakReference<Object> {
        final int hash;
        AIKey(Object k) {
            super(k);
            hash = System.identityHashCode(k);
        }
    }

    private static final int AI_CAP_INIT = 1 << 13;   // 8192 slots
    private static AIKey[] AI_KEYS = new AIKey[AI_CAP_INIT];
    private static AIShadow[] AI_VALS = new AIShadow[AI_CAP_INIT];
    private static int AI_SIZE = 0;      // live + dead entries
    private static int AI_DEAD = 0;      // GC-cleared entries seen while probing

    private static AIShadow shadowOf(Object gs) {
        int mask = AI_KEYS.length - 1;
        int h = System.identityHashCode(gs);
        int i = h & mask;
        while (true) {
            AIKey k = AI_KEYS[i];
            if (k == null) {
                AIShadow sh = new AIShadow();
                AI_KEYS[i] = new AIKey(gs);
                AI_VALS[i] = sh;
                AI_SIZE++;
                if (AI_SIZE + (AI_SIZE >> 1) >= AI_KEYS.length) {
                    aiRebuild(AI_KEYS.length << 1);
                }
                return sh;
            }
            Object cur = k.get();
            if (cur == gs) return AI_VALS[i];
            if (cur == null) {
                // dead slot: claim in place. Safe under linear probing —
                // the incoming key's probe path starts here, so the new
                // pair is found first; the dead key is unreachable by
                // definition; every other chain still terminates at a
                // null or its own live key.
                AI_DEAD++;
                AIShadow sh = new AIShadow();
                AI_KEYS[i] = new AIKey(gs);
                AI_VALS[i] = sh;
                if (AI_DEAD > 2048 && AI_DEAD * 2 > AI_SIZE) {
                    aiRebuild(AI_KEYS.length); // sweep dead, same capacity
                }
                return sh;
            }
            i = (i + 1) & mask;
        }
    }

    /** Non-inserting probe (the odd-tick fast path: a selector whose floor
     *  tick built the shadow already hits; a fresh selector returns null and
     *  the caller builds it with the full pass). Dead slots are tombstones:
     *  the probe must WALK PAST them (a live entry later in the chain is
     *  still reachable), terminating at the first true null. */
    private static AIShadow aiShadowPeek(Object gs) {
        AIKey[] keys = AI_KEYS;
        AIShadow[] vals = AI_VALS;
        int mask = keys.length - 1;
        int i = System.identityHashCode(gs) & mask;
        while (true) {
            AIKey k = keys[i];
            if (k == null) return null;
            Object cur = k.get();
            if (cur == gs) return vals[i];
            i = (i + 1) & mask;
        }
    }

    /** Rehash (capacity 0 = same capacity, dead sweep only). Keeps the
     *  live pairs, drops the dead ones. Called from the server thread —
     *  no racing readers. */
    private static void aiRebuild(int newCap) {
        AIKey[] oldKeys = AI_KEYS;
        AIShadow[] oldVals = AI_VALS;
        AIKey[] nk = new AIKey[newCap];
        AIShadow[] nv = new AIShadow[newCap];
        int mask = newCap - 1;
        int live = 0;
        for (int j = 0; j < oldKeys.length; j++) {
            AIKey k = oldKeys[j];
            if (k == null) continue;
            Object cur = k.get();
            if (cur == null) continue; // dead — dropped
            int i = k.hash & mask;
            while (nk[i] != null) i = (i + 1) & mask;
            nk[i] = k;
            nv[i] = oldVals[j];
            live++;
        }
        AI_KEYS = nk;
        AI_VALS = nv;
        AI_SIZE = live;
        AI_DEAD = 0;
    }

    private static final ThreadLocal<net.minecraft.world.entity.ai.goal.WrappedGoal[]> AI_LOCK_TABLE =
            ThreadLocal.withInitial(
                () -> new net.minecraft.world.entity.ai.goal.WrappedGoal[32]);

    /** Precompute-time proof: the private-handle set must resolve BEFORE
     *  the GoalSelector bodies are replaced (doc-25 rule — the mirror has
     *  no vanilla escape, so a lazy handle failure would crash the first
     *  mob tick). */
    public static void initAIHandles() {
        try {
            VH_GS_DISABLED = new RefField(
                    net.minecraft.world.entity.ai.goal.GoalSelector.class, "goalTypes");
            VH_GS_LOCKED = new RefField(
                    net.minecraft.world.entity.ai.goal.GoalSelector.class, "lockedFlags");
            FLAG_N = net.minecraft.world.entity.ai.goal.Goal.Flag.values().length;
            if (FLAG_N < 1 || FLAG_N > 32) {
                throw new IllegalStateException("unexpected Flag enum size: " + FLAG_N);
            }
            AI_HANDLES_OK = true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initAIHandles: " + t;
            AI_HANDLES_OK = false;
        }
    }

    private static long disabledBacking(net.minecraft.world.entity.ai.goal.GoalSelector gs) {
        ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet disabled =
                (ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet)
                        VH_GS_DISABLED.getObj(gs);
        return disabled.getBackingSet();
    }

    private static boolean canReplaceAll(
            net.minecraft.world.entity.ai.goal.WrappedGoal[] lock,
            net.minecraft.world.entity.ai.goal.WrappedGoal cand) {
        // L25: one getFlags() read (the old code called it twice — the
        // delegate chain is a virtual hop onto the inner goal per call).
        net.minecraft.world.entity.ai.goal.WrappedGoal g = cand;
        long fb = g.getFlags().getBackingSet();
        int fs = g.getFlags().size();
        for (int i = 0; i < fs; i++) {
            int ord = Long.numberOfTrailingZeros(fb);
            fb &= fb - 1;
            net.minecraft.world.entity.ai.goal.WrappedGoal holder = lock[ord];
            if (holder != null && !holder.canBeReplacedBy(g)) return false;
        }
        return true;
    }

    /** tickRunningGoals body shared by the force tail of tick() and the
     *  direct weave. force=true walks the full live set (vanilla-identical);
     *  force=false walks the running shadow with live re-checks. */
    private static void tickRunningImpl(net.minecraft.world.entity.ai.goal.GoalSelector gs,
                                        boolean force) {
        net.minecraft.util.profiling.ProfilerFiller profiler =
                net.minecraft.util.profiling.Profiler.get();
        profiler.push("goalTick");
        if (force) {
            for (net.minecraft.world.entity.ai.goal.WrappedGoal g : gs.getAvailableGoals()) {
                if (g.isRunning()) g.tick();
            }
        } else {
            gsFloorRunning++;
            AIShadow sh = aiShadowPeek(gs);
            if (sh == null) {
                // first odd-tick visit of a fresh selector: build the shadow
                // with a silent full pass (isRunning reads only — no calls)
                sh = shadowOf(gs);
                sh.n = 0;
                for (net.minecraft.world.entity.ai.goal.WrappedGoal g : gs.getAvailableGoals()) {
                    if (g.isRunning()) {
                        if (sh.n == sh.goals.length) {
                            sh.goals = java.util.Arrays.copyOf(sh.goals, sh.n * 2);
                        }
                        sh.goals[sh.n++] = g;
                    }
                }
            }
            int w = 0;
            for (int i = 0; i < sh.n; i++) {
                net.minecraft.world.entity.ai.goal.WrappedGoal g = sh.goals[i];
                if (g.isRunning()) {
                    sh.goals[w++] = g;
                    if (g.requiresUpdateEveryTick()) g.tick();
                }
            }
            sh.n = w; // prune the skipped tails
        }
        profiler.pop();
    }

    /** The exact 4-pass tick (the L15 v1 body) — runs when the fast-path
     *  shape validation fails (fresh shadow, a set mutation the merge-walk
     *  cannot pin, an externally started goal) or when the collide check
     *  fires. Ends by recording the shadow shape so the next tick can take
     *  the flat path again. */
    private static void fullPassTick(net.minecraft.world.entity.ai.goal.GoalSelector gs,
            java.util.Set<net.minecraft.world.entity.ai.goal.WrappedGoal> goals,
            AIShadow sh) {
        try {
            net.minecraft.util.profiling.ProfilerFiller profiler =
                    net.minecraft.util.profiling.Profiler.get();
            long disabled = disabledBacking(gs);
            // cleanup — vanilla conditions and short-circuit order
            profiler.push("goalCleanup");
            for (net.minecraft.world.entity.ai.goal.WrappedGoal g : goals) {
                if (g.isRunning()
                        && (((g.getFlags().getBackingSet() & disabled) != 0L)
                            || !g.canContinueToUse())) {
                    g.stop();
                    gsFloorStops++;
                }
            }
            profiler.pop();
            // rebuild the lock table + running shadow (silent pass)
            net.minecraft.world.entity.ai.goal.WrappedGoal[] lock = AI_LOCK_TABLE.get();
            for (int i = 0; i < FLAG_N; i++) lock[i] = null;
            sh.n = 0;
            for (net.minecraft.world.entity.ai.goal.WrappedGoal g : goals) {
                if (g.isRunning()) {
                    long fb = g.getFlags().getBackingSet();
                    int fs = g.getFlags().size();
                    for (int i = 0; i < fs; i++) {
                        int ord = Long.numberOfTrailingZeros(fb);
                        fb &= fb - 1;
                        if (lock[ord] != null) {
                            // vanilla's lock table can never hold two running
                            // goals on one flag; a plugin goal that mutates
                            // flags while running could — disarm, stay exact.
                            gsFloorCollide++;
                            AI_ON = false;
                            gsMirrorTick(gs);
                            return;
                        }
                        lock[ord] = g;
                    }
                    if (sh.n == sh.goals.length) {
                        sh.goals = java.util.Arrays.copyOf(sh.goals, sh.n * 2);
                    }
                    sh.goals[sh.n++] = g;
                }
            }
            // update — vanilla conditions and short-circuit order
            profiler.push("goalUpdate");
            for (net.minecraft.world.entity.ai.goal.WrappedGoal g : goals) {
                if (!g.isRunning()) {
                    long fb = g.getFlags().getBackingSet();
                    if ((fb & disabled) == 0L && canReplaceAll(lock, g) && g.canUse()) {
                        long it = fb;
                        int fs = g.getFlags().size();
                        for (int i = 0; i < fs; i++) {
                            int ord = Long.numberOfTrailingZeros(it);
                            it &= it - 1;
                            net.minecraft.world.entity.ai.goal.WrappedGoal cur = lock[ord];
                            if (cur != null) {
                                cur.stop();
                                gsFloorStops++;
                            }
                            lock[ord] = g;
                        }
                        g.start();
                        gsFloorStarts++;
                    }
                }
            }
            profiler.pop();
            // vanilla tick() ends with tickRunningGoals(true) — full-set walk
            tickRunningImpl(gs, true);
            // record the shadow shape for the L25 flat path
            sh.allSize = goals.size();
            sh.first = firstOf(goals);
            sh.last = lastOf(goals);
            sh.full = false;
        } catch (Throwable t) {
            // The mirror path is for KILL-SWITCH states only; a live floor
            // tick must propagate exceptions exactly like vanilla (a goal
            // throwing is a vanilla crash too). Disarm so the next tick is
            // clean, then rethrow the original.
            AI_ON = false;
            lastError = System.nanoTime();
            lastErrorText = "gsFloor: " + t;
            if (t instanceof RuntimeException re) throw re;
            if (t instanceof Error err) throw err;
            throw new IllegalStateException("goalSelectorTick floor failed", t);
        }
    }

    @SuppressWarnings({"rawtypes"})
    private static Object firstOf(java.util.Set s) {
        // fastutil's first()/last() THROW NoSuchElementException on an empty
        // set (Brain-mobs tick an empty goalSelector every tick — the e2e
        // run died exactly there on the first floor tick).
        return s.isEmpty() ? null
                : (s instanceof it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet o)
                        ? o.first() : null;
    }

    @SuppressWarnings({"rawtypes"})
    private static Object lastOf(java.util.Set s) {
        return s.isEmpty() ? null
                : (s instanceof it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet o)
                        ? o.last() : null;
    }

    /** Replacement body of GoalSelector.tick() — the vanilla algorithm with
     *  flat-lock-table bookkeeping and the exact vanilla call sequence.
     *  L25: cleanup + lock-build + the tail run over the running-shadow
     *  array; the availableGoals set is iterated ONCE (the update pass)
     *  with a survivor merge-walk that proves the shadow matches the live
     *  set — any mismatch falls into fullPassTick for an exact rebuild. */
    public static void goalSelectorTick(net.minecraft.world.entity.ai.goal.GoalSelector gs) {
        if (!AI_ON || !AI_HANDLES_OK) {
            gsMirrorTick(gs);
            return;
        }
        gsFloorTicks++;
        try {
            java.util.Set<net.minecraft.world.entity.ai.goal.WrappedGoal> goals =
                    gs.getAvailableGoals();
            AIShadow sh = shadowOf(gs);
            if (sh.full
                    || sh.allSize != goals.size()
                    || sh.first != firstOf(goals)
                    || sh.last != lastOf(goals)) {
                fullPassTick(gs, goals, sh);
                return;
            }
            net.minecraft.util.profiling.ProfilerFiller profiler =
                    net.minecraft.util.profiling.Profiler.get();
            long disabled = disabledBacking(gs);
            // cleanup — vanilla conditions and short-circuit order, over the
            // running shadow (same relative set order; isRunning re-checked
            // per entry so an externally stopped goal skips exactly where
            // vanilla's set walk would skip it)
            profiler.push("goalCleanup");
            int w = 0;
            for (int i = 0; i < sh.n; i++) {
                net.minecraft.world.entity.ai.goal.WrappedGoal g = sh.goals[i];
                if (!g.isRunning()) continue;
                long fb = g.getFlags().getBackingSet();
                if (((fb & disabled) != 0L) || !g.canContinueToUse()) {
                    g.stop();
                    gsFloorStops++;
                    continue;
                }
                sh.goals[w++] = g;
            }
            sh.n = w;
            profiler.pop();
            // lock table from the survivors
            net.minecraft.world.entity.ai.goal.WrappedGoal[] lock = AI_LOCK_TABLE.get();
            for (int i = 0; i < FLAG_N; i++) lock[i] = null;
            for (int i = 0; i < w; i++) {
                net.minecraft.world.entity.ai.goal.WrappedGoal g = sh.goals[i];
                long fb = g.getFlags().getBackingSet();
                int fs = g.getFlags().size();
                for (int j = 0; j < fs; j++) {
                    int ord = Long.numberOfTrailingZeros(fb);
                    fb &= fb - 1;
                    if (lock[ord] != null) {
                        // two running goals on one flag — a plugin mutated
                        // flags while running: disarm, stay exact.
                        gsFloorCollide++;
                        AI_ON = false;
                        gsMirrorTick(gs);
                        return;
                    }
                    lock[ord] = g;
                }
            }
            // update — the ONLY full set pass. Survivor merge-walk: a running
            // set element must be the next expected survivor (both walk the
            // same insertion order); anything else (external start, direct
            // set surgery) falls to the exact 4-pass rebuild.
            profiler.push("goalUpdate");
            int n = w;
            int j = 0;
            for (net.minecraft.world.entity.ai.goal.WrappedGoal g : goals) {
                if (g.isRunning()) {
                    if (j < w && sh.goals[j] == g) {
                        j++;
                        continue;
                    }
                    profiler.pop();
                    fullPassTick(gs, goals, sh);
                    return;
                }
                long fb = g.getFlags().getBackingSet();
                if ((fb & disabled) == 0L && canReplaceAll(lock, g) && g.canUse()) {
                    long it = fb;
                    int fs = g.getFlags().size();
                    for (int k = 0; k < fs; k++) {
                        int ord = Long.numberOfTrailingZeros(it);
                        it &= it - 1;
                        net.minecraft.world.entity.ai.goal.WrappedGoal cur = lock[ord];
                        if (cur != null) {
                            cur.stop();
                            gsFloorStops++;
                        }
                        lock[ord] = g;
                    }
                    g.start();
                    gsFloorStarts++;
                    if (n == sh.goals.length) {
                        sh.goals = java.util.Arrays.copyOf(sh.goals, n * 2);
                    }
                    sh.goals[n++] = g;
                }
            }
            profiler.pop();
            if (j != w) {
                // a survivor vanished from the set mid-walk (direct set
                // surgery) — the exact rebuild pins the new shape
                fullPassTick(gs, goals, sh);
                return;
            }
            sh.n = n;
            // vanilla tail — tickRunningGoals(true): the running array IS the
            // set's running subset in the same order; every entry re-checked
            profiler.push("goalTick");
            for (int i = 0; i < n; i++) {
                net.minecraft.world.entity.ai.goal.WrappedGoal g = sh.goals[i];
                if (g.isRunning()) g.tick();
            }
            profiler.pop();
        } catch (Throwable t) {
            // The mirror path is for KILL-SWITCH states only; a live floor
            // tick must propagate exceptions exactly like vanilla (a goal
            // throwing is a vanilla crash too). Disarm so the next tick is
            // clean, then rethrow the original.
            AI_ON = false;
            lastError = System.nanoTime();
            lastErrorText = "gsFloor: " + t;
            if (t instanceof RuntimeException re) throw re;
            if (t instanceof Error err) throw err;
            throw new IllegalStateException("goalSelectorTick floor failed", t);
        }
    }

    /** Replacement body of GoalSelector.tickRunningGoals(boolean). */
    public static void goalSelectorTickRunning(
            net.minecraft.world.entity.ai.goal.GoalSelector gs, boolean force) {
        if (!AI_ON || !AI_HANDLES_OK) {
            gsMirrorTickRunning(gs, force);
            return;
        }
        try {
            tickRunningImpl(gs, force);
        } catch (Throwable t) {
            AI_ON = false;
            lastError = System.nanoTime();
            lastErrorText = "gsFloorRun: " + t;
            if (t instanceof RuntimeException re) throw re;
            if (t instanceof Error err) throw err;
            throw new IllegalStateException("goalSelectorTickRunning floor failed", t);
        }
    }

    // ---- mirrors: byte-exact vanilla ports (kill-switch + pre-arm path) ----

    @SuppressWarnings({"unchecked", "rawtypes"})
    private static void gsMirrorTick(net.minecraft.world.entity.ai.goal.GoalSelector gs) {
        gsMirrorTicks++;
        java.util.Set goals = gs.getAvailableGoals();
        java.util.EnumMap locked = (java.util.EnumMap) VH_GS_LOCKED.getObj(gs);
        ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet disabled =
                (ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet)
                        VH_GS_DISABLED.getObj(gs);
        net.minecraft.util.profiling.ProfilerFiller profiler =
                net.minecraft.util.profiling.Profiler.get();
        profiler.push("goalCleanup");
        for (Object o : goals) {
            net.minecraft.world.entity.ai.goal.WrappedGoal g =
                    (net.minecraft.world.entity.ai.goal.WrappedGoal) o;
            if (g.isRunning()
                    && (g.getFlags().hasCommonElements(disabled) || !g.canContinueToUse())) {
                g.stop();
            }
        }
        locked.entrySet().removeIf(
                e -> !((net.minecraft.world.entity.ai.goal.WrappedGoal)
                        ((java.util.Map.Entry) e).getValue()).isRunning());
        profiler.pop();
        profiler.push("goalUpdate");
        for (Object o : goals) {
            net.minecraft.world.entity.ai.goal.WrappedGoal g =
                    (net.minecraft.world.entity.ai.goal.WrappedGoal) o;
            if (!g.isRunning() && !g.getFlags().hasCommonElements(disabled)
                    && mirrorCanBeReplacedForAllFlags(g, locked) && g.canUse()) {
                long it = g.getFlags().getBackingSet();
                int fs = g.getFlags().size();
                for (int i = 0; i < fs; i++) {
                    int ord = Long.numberOfTrailingZeros(it);
                    it &= it - 1;
                    Object flag = goalFlags()[ord];
                    net.minecraft.world.entity.ai.goal.WrappedGoal cur =
                            (net.minecraft.world.entity.ai.goal.WrappedGoal)
                                    locked.getOrDefault(flag, null);
                    if (cur != null) cur.stop();
                    locked.put(flag, g);
                }
                g.start();
            }
        }
        profiler.pop();
        // vanilla tail: tickRunningGoals(true)
        profiler.push("goalTick");
        for (Object o : goals) {
            net.minecraft.world.entity.ai.goal.WrappedGoal g =
                    (net.minecraft.world.entity.ai.goal.WrappedGoal) o;
            if (g.isRunning()) g.tick();
        }
        profiler.pop();
    }

    @SuppressWarnings({"unchecked", "rawtypes"})
    private static boolean mirrorCanBeReplacedForAllFlags(
            net.minecraft.world.entity.ai.goal.WrappedGoal g, java.util.EnumMap locked) {
        long it = g.getFlags().getBackingSet();
        int fs = g.getFlags().size();
        for (int i = 0; i < fs; i++) {
            int ord = Long.numberOfTrailingZeros(it);
            it &= it - 1;
            Object flag = goalFlags()[ord];
            net.minecraft.world.entity.ai.goal.WrappedGoal cur =
                    (net.minecraft.world.entity.ai.goal.WrappedGoal)
                            locked.getOrDefault(flag, null);
            // vanilla: getOrDefault(flag, NO_GOAL) — NO_GOAL is replaceable
            if (cur != null && !cur.canBeReplacedBy(g)) return false;
        }
        return true;
    }

    private static void gsMirrorTickRunning(net.minecraft.world.entity.ai.goal.GoalSelector gs,
                                            boolean force) {
        net.minecraft.util.profiling.ProfilerFiller profiler =
                net.minecraft.util.profiling.Profiler.get();
        profiler.push("goalTick");
        for (net.minecraft.world.entity.ai.goal.WrappedGoal g : gs.getAvailableGoals()) {
            if (g.isRunning() && (force || g.requiresUpdateEveryTick())) g.tick();
        }
        profiler.pop();
    }

    // ==================================================================
    // L15 — micro-gates on pure no-op writes (doc 26).
    //
    // setTicksFrozen: the vanilla body is exactly
    // `entityData.set(DATA_TICKS_FROZEN, Integer.valueOf(i))` (javap) and
    // SynchedEntityData.set already no-ops the write when the value equals
    // (ObjectUtils.notEqual gate, onSyncedDataUpdated skipped). The profile
    // still shows 1.83% self time — the whole inlined chain (getItem +
    // notEqual + boxing) paid per entity per tick for the same value. The
    // gate compares the live data value first; on equality the body is a
    // provable no-op and is skipped whole.
    //
    // ConversionTracker.setConverting: same shape — every non-afflicted mob
    // pays `getEntityData().set(conversionId, false)` every tick (1.26%
    // self in mobs-25k-mod). The gate reads the live Boolean (cached
    // TRUE/FALSE — no boxing) and skips the equal write.
    // Both bridges keep the exact vanilla write when the value differs, and
    // are armed only after their handle sets are proven at precompute time.
    // ==================================================================

    public static volatile boolean FROZEN_ON = false;
    public static volatile boolean FROZEN_HANDLES_OK = false;
    public static volatile boolean CONV_ON = false;
    public static volatile boolean CONV_HANDLES_OK = false;
    // liveness counters: bumped on EVERY bridge entry (the verifier proves
    // the weave by watching these BEFORE the gates are armed — frozenGate/
    // convGate move only after arming). PLAIN longs, same reasoning as gsFloorTicks.
    // frozenDirect/convDirect = the L25 direct-read layer's catches (the
    // proven get-chain layer counts into the same frozenGate/convGate).
    public static long frozenCalls = 0, convCalls = 0;
    public static long frozenGate = 0, frozenWrites = 0, frozenDirect = 0;
    public static long convGate = 0, convWrites = 0, convDirect = 0;
    // system-property kill switches, folded to static finals at class init.
    // NOTE the idiom: Boolean.getBoolean(name) returns FALSE when the
    // property is ABSENT — the first CI e2e armed with the floor disabled
    // because of exactly that. Default-ON must read the property explicitly.
    public static final boolean AI_FLAG_ON =
            !"false".equals(System.getProperty("centities.aiFloor", "true"));
    public static final boolean FROZEN_FLAG_ON =
            !"false".equals(System.getProperty("centities.frozenGate", "true"));
    public static final boolean CONV_FLAG_ON =
            !"false".equals(System.getProperty("centities.convGate", "true"));

    private static volatile RefField VH_ENT_DATA;  // Entity.entityData
    private static volatile RefField VH_CONV_MOB;  // ConversionTracker.mob
    private static volatile RefField VH_CONV_ID;   // ConversionTracker.conversionId

    public static void initFrozenHandles() {
        try {
            VH_ENT_DATA = new RefField(Entity.class, "entityData");
            initDataOffsets();
            // L25: the direct entityData offset for the fast gates (the
            // offset is per-class constant; VH_ENT_DATA.off never changes).
            if (VH_ENT_DATA.off >= 0) {
                O_ED = VH_ENT_DATA.off;
            }
            // L29: freeze the static accessor indices once the offsets are
            // in. The reads below are the SAME capture pattern the flag
            // gate uses for DATA_SHARED_FLAGS_ID (static-field Unsafe read
            // — identity preserved), and each accessor is instanceof'd
            // before its record component is read.
            if (U != null && O_ED >= 0 && O_ED_ITEMS >= 0 && O_DI_VALUE >= 0) {
                try {
                    java.lang.reflect.Field ff = Entity.class
                            .getDeclaredField("DATA_TICKS_FROZEN");
                    ff.setAccessible(true);
                    Object fa = U.getObject(U.staticFieldBase(ff),
                            U.staticFieldOffset(ff));
                    if (fa instanceof net.minecraft.network.syncher.EntityDataAccessor) {
                        IDX_FROZEN = ((net.minecraft.network.syncher.EntityDataAccessor<?>) fa).id();
                    }
                } catch (Throwable frozenIdx) {
                    IDX_FROZEN = -1;
                }
                if (FLAGS_ID instanceof net.minecraft.network.syncher.EntityDataAccessor) {
                    IDX_FLAGS = ((net.minecraft.network.syncher.EntityDataAccessor<?>) FLAGS_ID).id();
                }
                // FLAGS_ID is captured by initFlagHandles which may run
                // after us; EDV_OK then flips in initFlagHandles too.
                if (IDX_FROZEN >= 0 && IDX_FLAGS >= 0) {
                    EDV_OK = true;
                }
            }
            FROZEN_HANDLES_OK = true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initFrozenHandles: " + t;
            FROZEN_HANDLES_OK = false;
        }
    }

    /** L25 — direct-offset fast-gate infrastructure. O_ED = Entity.entityData;
     *  O_CT_MOB/O_CT_ID = ConversionTracker.mob/conversionId; O_ACC_ID =
     *  EntityDataAccessor.id. All single-class offsets per the doc-32 rule:
     *  every fast path re-checks the receiver's type before the next hop,
     *  and any miss routes the exact vanilla chain. */
    private static long O_ED = -1;
    private static long O_CT_MOB = -1, O_CT_ID = -1, O_ACC_ID = -1;

    // L29 — precomputed data-array indices for the STATIC accessors. An
    // EntityDataAccessor's id is assigned once in the owner's <clinit> and
    // never reassigned, and the SynchedEntityData.items array is indexed by
    // exactly that id (entityDataValue javap-ground since L25), so the id
    // is a run-constant int. Freezing it kills the per-call record
    // accessor call + the accessor instanceof on every gate read. The
    // identity is re-verified at init (the accessor read from the class
    // must itself be an EntityDataAccessor) and the lean reader below
    // still bounds-checks idx against the live array length, so a
    // hypothetical layout change degrades to the vanilla chain, never to
    // a wrong read.
    private static int IDX_FROZEN = -1, IDX_FLAGS = -1;
    /** true when O_ED/O_ED_ITEMS/O_DI_VALUE resolved AND the two static
     *  indices froze — the lean reader's single readiness gate. */
    private static volatile boolean EDV_OK = false;
    // L29-converting: the ConversionTracker's accessor field holds the
    // same static accessor for every tracker of a mob type; a one-slot
    // identity cache skips the instanceof + record call on the hot repeat.
    private static Object CONV_ACC_CACHE;
    private static int CONV_ACC_IDX = -1;

    /** The current value of the accessor `acc` on the entity `e` — the
     *  direct-read gate core. Returns null when any shape check fails (the
     *  caller then routes the vanilla chain). Typed at every hop: the
     *  receiver of each Unsafe read is instanceof-checked against the class
     *  the offset belongs to (doc-32 rule — the SE_FAST lesson). */
    private static Object entityDataValue(Entity e, Object acc) {
        if (O_ED < 0 || O_ED_ITEMS < 0 || O_DI_VALUE < 0) {
            // L28c one-shot: the FIRST null explanation (the ab legs could
            // not tell WHICH hop fails — every gate counter is shared by
            // both layers and the profile only shows the chain's cost).
            if (!mEdvProbed) {
                mEdvProbed = true;
                lastErrorText = "edv: off ED=" + O_ED + " ITEMS=" + O_ED_ITEMS
                        + " DIV=" + O_DI_VALUE + " ACC=" + O_ACC_ID;
            }
            return null;
        }
        try {
            Object ed = U.getObject(e, O_ED);
            if (!(ed instanceof net.minecraft.network.syncher.SynchedEntityData sde)) {
                return edvProbe("ed=" + ed);
            }
            if (!(acc instanceof net.minecraft.network.syncher.EntityDataAccessor acc0)) {
                return edvProbe("acc=" + acc);
            }
            Object[] items = (Object[]) U.getObject(sde, O_ED_ITEMS);
            // L28d (the edv probe's verdict, reproduced locally on JDK 25:
            // "UnsupportedOperationException: can't get field offset on a
            // record class") — sun.misc.Unsafe REFUSES record fields, so the
            // O_ACC_ID offset can never resolve and the direct layer silently
            // dead-fell into the chain layer everywhere. The accessor is a
            // RECORD: its public id() accessor is the exact read, JIT-inlined,
            // no Unsafe needed.
            int idx = acc0.id();
            if (idx < 0 || idx >= items.length) {
                return edvProbe("idx=" + idx + " len=" + items.length);
            }
            Object di = items[idx];
            if (!(di instanceof net.minecraft.network.syncher.SynchedEntityData.DataItem)) {
                return edvProbe("di=" + di);
            }
            return U.getObject(di, O_DI_VALUE);
        } catch (Throwable t) {
            return edvProbe("throw " + t);
        }
    }

    private static volatile boolean mEdvProbed = false;

    private static Object edvProbe(String what) {
        if (!mEdvProbed) {
            mEdvProbed = true;
            lastErrorText = "edv: " + what;
        }
        return null;
    }

    /** L29 — the lean direct read for the STATIC accessors whose array
     *  index is frozen (IDX_*). Same loads as entityDataValue: the
     *  Entity.entityData hop (offset belongs to Entity, the receiver is
     *  the weave-guaranteed `this` of the replaced setter — the field's
     *  declared type is SynchedEntityData, so the null check is the full
     *  value-type check the JVM typing already guarantees), the items[]
     *  hop (checkcast kept), the bounds-checked slot load, the DataItem
     *  instanceof (the element type is NOT declared — this check stays),
     *  the value hop. Returns null on any structural doubt — the caller
     *  routes the vanilla chain exactly as with entityDataValue. */
    private static Object entityDataValueIdx(Entity e, int idx) {
        if (!EDV_OK) {
            return null;
        }
        try {
            Object ed = U.getObject(e, O_ED);
            if (!(ed instanceof net.minecraft.network.syncher.SynchedEntityData sde)) {
                return edvProbe("idx ed=" + ed);
            }
            Object[] items = (Object[]) U.getObject(sde, O_ED_ITEMS);
            if (idx < 0 || idx >= items.length) {
                return edvProbe("idx=" + idx + " len=" + items.length);
            }
            Object di = items[idx];
            if (!(di instanceof net.minecraft.network.syncher.SynchedEntityData.DataItem)) {
                return edvProbe("idx di=" + di);
            }
            return U.getObject(di, O_DI_VALUE);
        } catch (Throwable t) {
            return edvProbe("idx throw " + t);
        }
    }

    // ===================================================================
    // L18 — the setSharedFlag gate. Entity.setSharedFlag (javap, 26.3) is a
    // data read + conditional write of ONE byte; SynchedEntityData.set
    // already skips equal values, so the whole body is a no-op whenever the
    // computed byte equals the stored byte — which is EVERY tick for the
    // flags a quiet mob re-asserts (onFire/crouching/glowing). The gate
    // computes the new byte first and only pays the boxed set call on a
    // real change. The accessor is protected (unlike the public
    // DATA_TICKS_FROZEN) — it is captured ONCE via an Unsafe static-field
    // read at handle-init; identity is preserved (the data map keys by the
    // accessor instance), so reads/writes are the vanilla ones.
    // ===================================================================
    public static volatile boolean FLAGG_ON = false;
    public static volatile boolean FLAGG_HANDLES_OK = false;
    public static final boolean FLAGG_FLAG_ON =
            !"false".equals(System.getProperty("centities.flagGate", "true"));
    public static long flagCalls = 0, flagGate = 0, flagWrites = 0;
    private static Object FLAGS_ID; // Entity.DATA_SHARED_FLAGS_ID (captured)

    public static void initFlagHandles() {
        try {
            if (VH_ENT_DATA == null) {
                VH_ENT_DATA = new RefField(Entity.class, "entityData");
            }
            if (U != null) {
                java.lang.reflect.Field f =
                        Entity.class.getDeclaredField("DATA_SHARED_FLAGS_ID");
                f.setAccessible(true);
                FLAGS_ID = U.getObject(U.staticFieldBase(f), U.staticFieldOffset(f));
            }
            if (FLAGS_ID != null) {
                FLAGG_HANDLES_OK = true;
                // L29: the flags index may freeze here when initFlagHandles
                // runs before initFrozenHandles (or re-freeze after).
                if (U != null && IDX_FROZEN >= 0 && O_ED >= 0
                        && O_ED_ITEMS >= 0 && O_DI_VALUE >= 0
                        && FLAGS_ID instanceof
                                net.minecraft.network.syncher.EntityDataAccessor) {
                    IDX_FLAGS = ((net.minecraft.network.syncher.EntityDataAccessor<?>)
                            FLAGS_ID).id();
                    if (IDX_FLAGS >= 0) {
                        EDV_OK = true;
                    }
                }
            }
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initFlagHandles: " + t;
            FLAGG_HANDLES_OK = false;
        }
    }

    public static void setFlagGateOn(boolean on) {
        FLAGG_ON = on;
    }

    // ==================================================================
    // L26 — the Sensing floor. Vanilla Sensing.tick() (26.3, decompiled)
    // is `seen.clear(); unseen.clear();` — and fastutil IntOpenHashSet
    // .clear() is an O(capacity) Arrays.fill whenever the set is non-empty
    // (the bench mobs' LookAtPlayer scans fill it every tick: 2.2% of the
    // mobs-25k-mod profile on the two clear() lines alone). The floor
    // swaps the two PRIVATE final IntSet fields (seen/unseen — only
    // Sensing.tick/hasLineOfSight ever touch them, decompile-verified) for
    // GenIntSet instances: an open-addressed int table with per-slot
    // generation stamps, whose clear() is ONE integer increment. The
    // vanilla hasLineOfSight body stays untouched — it reads the fields
    // and calls contains/add on whatever IntSet is there, and GenIntSet
    // honors the IntSet contract for exactly those operations. The swap
    // stores a real reference into a declared IntSet field via
    // Unsafe.putObject — a proper oop store (the SE_FAST bug class was a
    // raw offset READ of a foreign-class receiver, not a real-reference
    // store; doc 32's rule is honored: every hop here is typed).
    // ==================================================================

    public static volatile boolean SENSE_ON = false;
    public static volatile boolean SENSE_HANDLES_OK = false;
    public static final boolean SENSE_FLAG_ON =
            !"false".equals(System.getProperty("centities.sensing", "true"));
    public static long senseCalls = 0, senseSwaps = 0;
    private static volatile RefField VH_SN_SEEN;  // Sensing.seen
    private static volatile RefField VH_SN_UNSEEN; // Sensing.unseen
    private static long O_SN_SEEN = -1, O_SN_UNSEEN = -1;

    public static void initSenseHandles() {
        try {
            VH_SN_SEEN = new RefField(net.minecraft.world.entity.ai.sensing.Sensing.class, "seen");
            VH_SN_UNSEEN = new RefField(
                    net.minecraft.world.entity.ai.sensing.Sensing.class, "unseen");
            if (U != null && VH_SN_SEEN.off >= 0 && VH_SN_UNSEEN.off >= 0) {
                O_SN_SEEN = VH_SN_SEEN.off;
                O_SN_UNSEEN = VH_SN_UNSEEN.off;
            }
            SENSE_HANDLES_OK = true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initSenseHandles: " + t;
            SENSE_HANDLES_OK = false;
        }
    }

    public static void setSenseOn(boolean on) {
        SENSE_ON = on;
    }

    /** Replacement body of Sensing.tick() — first call per Sensing swaps
     *  the fields to GenIntSet; every call then clears O(1). The kill-switch
     *  mirror clears whatever IntSet sits in the fields (GenIntSet included
     *  — its clear() works through the interface), so a mid-run disarm
     *  stays exact. */
    public static void sensingTick$(net.minecraft.world.entity.ai.sensing.Sensing s) {
        senseCalls++;
        if (SENSE_ON && SENSE_HANDLES_OK && O_SN_SEEN >= 0) {
            Object a = U.getObject(s, O_SN_SEEN);
            Object b = U.getObject(s, O_SN_UNSEEN);
            if (a instanceof GenIntSet ga) {
                ga.clear();
                if (b instanceof GenIntSet gb) {
                    gb.clear();
                    return;
                }
                if (b instanceof java.util.Set<?> bs) {
                    bs.clear();
                    return;
                }
                // unknown unseen type — vanilla body via the field read
                U.getObject(s, O_SN_UNSEEN);
                return;
            }
            // first sight: install both (a fresh Sensing's sets are empty —
            // nothing to preserve, and a mid-life swap is exact too: the
            // generation stamp starts at 1 = "empty")
            U.putObject(s, O_SN_SEEN, new GenIntSet());
            U.putObject(s, O_SN_UNSEEN, new GenIntSet());
            senseSwaps++;
            return;
        }
        try {
            Object a = VH_SN_SEEN.getObj(s);
            if (a instanceof java.util.Set<?> sa) sa.clear();
            Object b = VH_SN_UNSEEN.getObj(s);
            if (b instanceof java.util.Set<?> sb) sb.clear();
        } catch (Throwable mirrorFail) {
            SENSE_HANDLES_OK = false;
            lastError = System.nanoTime();
            lastErrorText = "senseMirror: " + mirrorFail;
        }
    }

    /** The drop-in IntSet: O(1) clear via per-slot generation stamps, open
     *  addressing with linear probing. Entity ids never collide with the
     *  0 empty marker (vanilla's ENTITY_COUNTER starts at 1), but id 0 is
     *  still handled through an explicit slot. Stale-generation entries
     *  stay as tombstones until the next grow() rehashes them away — probe
     *  chains always terminate (capacity is grown at 75% occupancy of
     *  non-empty slots). Vanilla only ever calls clear/contains/add on
     *  these sets (Sensing decompile); the remaining IntSet surface is
     *  implemented faithfully anyway. */
    public static final class GenIntSet extends it.unimi.dsi.fastutil.ints.AbstractIntSet {
        private int[] keys = new int[16];
        private int[] gens = new int[16];
        private int mask = 15;
        private int gen = 1;
        private int used = 0;      // non-empty slots (incl. stale tombstones)
        private int live = 0;      // current-generation entries
        private int threshold = 12;
        private boolean hasZero;
        private int zeroGen;

        private static int mix(int k) {
            k *= 0x9E3779B9;
            return k ^ (k >>> 16);
        }

        @Override
        public void clear() {
            if (gen == Integer.MAX_VALUE) {
                java.util.Arrays.fill(keys, 0);
                gen = 1;
            } else {
                gen++;
            }
            // CRITICAL: `used` must NOT be reset here — the keys array keeps
            // the stale ids (probe-visible tombstones), so the non-empty
            // slot count is unchanged. Zeroing it would let `add` exceed the
            // capacity before the next grow and the probe loop would never
            // find an empty slot (a hang). The stale entries stay invisible
            // to contains() (gen mismatch) and are rehashed away on growth.
            live = 0;
            hasZero = false;
        }

        @Override
        public boolean contains(int k) {
            if (k == 0) return hasZero && zeroGen == gen;
            int i = mix(k) & mask;
            while (true) {
                int kk = keys[i];
                if (kk == 0) return false;
                if (kk == k && gens[i] == gen) return true;
                i = (i + 1) & mask;
            }
        }

        @Override
        public boolean add(int k) {
            if (k == 0) {
                if (hasZero && zeroGen == gen) return false;
                if (!hasZero) used++;
                hasZero = true;
                zeroGen = gen;
                live++;
                if (used > threshold) grow();
                return true;
            }
            int i = mix(k) & mask;
            while (true) {
                int kk = keys[i];
                if (kk == 0) {
                    keys[i] = k;
                    gens[i] = gen;
                    used++;
                    live++;
                    if (used > threshold) grow();
                    return true;
                }
                if (kk == k) {
                    if (gens[i] == gen) return false;
                    gens[i] = gen;   // reactivate a stale tombstone
                    live++;
                    return true;
                }
                i = (i + 1) & mask;
            }
        }

        @Override
        public boolean remove(int k) {
            if (k == 0) {
                if (hasZero && zeroGen == gen) {
                    hasZero = false;
                    live--;
                    return true;
                }
                return false;
            }
            int i = mix(k) & mask;
            while (true) {
                int kk = keys[i];
                if (kk == 0) return false;
                if (kk == k) {
                    if (gens[i] == gen) {
                        gens[i] = 0;  // tombstone: invisible, probe-safe
                        live--;
                        return true;
                    }
                    return false;
                }
                i = (i + 1) & mask;
            }
        }

        @Override
        public int size() {
            return live;
        }

        @Override
        public boolean isEmpty() {
            return live == 0;
        }

        private void grow() {
            int[] ok = keys;
            int[] og = gens;
            int nc = keys.length << 1;
            keys = new int[nc];
            gens = new int[nc];
            mask = nc - 1;
            threshold = nc - (nc >> 2);
            int u = 0;
            int l = 0;
            for (int i = 0; i < ok.length; i++) {
                if (ok[i] != 0 && og[i] == gen) {
                    int j = mix(ok[i]) & mask;
                    while (keys[j] != 0) j = (j + 1) & mask;
                    keys[j] = ok[i];
                    gens[j] = gen;
                    u++;
                    l++;
                }
            }
            if (hasZero && zeroGen == gen) {
                l++;
            }
            used = u;
            live = l;
        }

        @Override
        public it.unimi.dsi.fastutil.ints.IntIterator iterator() {
            int[] k = keys;
            int[] g = gens;
            int cur = gen;
            return new it.unimi.dsi.fastutil.ints.IntIterator() {
                private int i = 0;
                private int next = -1;
                private boolean has = false;

                private void advance() {
                    while (!has && i < k.length) {
                        if (k[i] != 0 && g[i] == cur) {
                            next = k[i];
                            has = true;
                        }
                        i++;
                    }
                }

                @Override
                public boolean hasNext() {
                    advance();
                    return has;
                }

                @Override
                public int nextInt() {
                    advance();
                    if (!has) throw new java.util.NoSuchElementException();
                    has = false;
                    return next;
                }
            };
        }
    }

    // ==================================================================
    // L26 — the getSharedFlag floor. Entity.getSharedFlag(int) (javap,
    // 26.3) is entityData.get(DATA_SHARED_FLAGS_ID) -> Byte -> (1<<flag)&b
    // — the full boxed get chain (getItem + DataItem.getValue + cast +
    // unbox) for a read that happens ~10-30x per mob per tick across
    // isOnFire/isCrouching/isSprinting/isGlowing/... The floor replaces the
    // body with the same read off the DataItem directly (typed at every
    // hop, single-class offsets per the doc-32 rule). The mirror is the
    // byte-exact vanilla chain.
    // ==================================================================

    public static volatile boolean FLAGR_ON = false;
    public static volatile boolean FLAGR_HANDLES_OK = false;
    public static final boolean FLAGR_FLAG_ON =
            !"false".equals(System.getProperty("centities.flagRead", "true"));
    public static long flagrCalls = 0, flagrHits = 0;
    private static int FLAGR_ACC_ID = -1;

    public static void initFlagRHandles() {
        try {
            if (FLAGS_ID == null) {
                initFlagHandles();
            }
            if (!FLAGG_HANDLES_OK || FLAGS_ID == null || O_ED_ITEMS < 0 || O_DI_VALUE < 0) {
                FLAGR_HANDLES_OK = false;
                return;
            }
            FLAGR_ACC_ID = ((net.minecraft.network.syncher.EntityDataAccessor<?>) FLAGS_ID).id();
            if (VH_ENT_DATA == null) {
                VH_ENT_DATA = new RefField(Entity.class, "entityData");
            }
            if (O_ED < 0 && VH_ENT_DATA.off >= 0) {
                O_ED = VH_ENT_DATA.off;
            }
            if (O_ACC_ID < 0) {
                initDataOffsets();
            }
            FLAGR_HANDLES_OK = FLAGR_ACC_ID >= 0 && O_ED >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initFlagRHandles: " + t;
            FLAGR_HANDLES_OK = false;
        }
    }

    public static void setFlagROn(boolean on) {
        FLAGR_ON = on;
    }

    /** Replacement body of Entity.getSharedFlag(int) — the direct DataItem
     *  read; any shape-check miss routes the byte-exact vanilla chain. */
    public static boolean getSharedFlag$(Entity e, int flag) {
        flagrCalls++;
        if (FLAGR_ON && FLAGR_HANDLES_OK) {
            // L29: the frozen-index read first; the accessor walk stays as
            // the structural fallback for a lean miss.
            Object cur = entityDataValueIdx(e, IDX_FLAGS);
            if (cur == null) {
                cur = entityDataValue(e, FLAGS_ID);
            }
            if (cur instanceof Byte b0) {
                flagrHits++;
                return ((1 << flag) & b0.byteValue()) != 0;
            }
        }
        // vanilla mirror — byte-exact port of the 26.3 body
        try {
            net.minecraft.network.syncher.SynchedEntityData ed =
                    (net.minecraft.network.syncher.SynchedEntityData) VH_ENT_DATA.getObj(e);
            @SuppressWarnings("unchecked")
            net.minecraft.network.syncher.EntityDataAccessor<Byte> id =
                    (net.minecraft.network.syncher.EntityDataAccessor<Byte>) flagsId();
            Byte b = ed.get(id);
            return ((1 << flag) & b.byteValue()) != 0;
        } catch (Throwable mirrorFail) {
            FLAGR_HANDLES_OK = false;
            lastError = System.nanoTime();
            lastErrorText = "flagRMirror: " + mirrorFail;
            if (mirrorFail instanceof RuntimeException re) throw re;
            if (mirrorFail instanceof Error err) throw err;
            throw new IllegalStateException("getSharedFlag mirror failed", mirrorFail);
        }
    }

    // ==================================================================
    // L22a — the native AI floor (Mob.serverAiStep orchestration)
    //
    // Woven EarlyReturnGuard on Mob.serverAiStep(): the whole per-mob AI
    // sequence — noActionTime++, sensing clear, the target/goal selector
    // parity split, navigation, controls — is driven by the RUST engine
    // (centities-core/src/goals.rs) through the packed-buffer protocol.
    // The engine reproduces the vanilla GoalSelector sequences EXACTLY
    // (insertion order, flag math, priority replacement, the shared-RNG
    // consumption with the seed handed back and forth) and stages every
    // decision it cannot prove as a REQUEST resolved with the real virtual
    // call. start()/stop()/tick() of unported goal bodies always run as the
    // real virtual calls; the engine only keeps the lock book.
    //
    // The RNG is Entity.SHARED_RANDOM — one ThreadUnsafeRandom LCG shared
    // by every entity. The engine advances it natively (bit-exact with the
    // JDK, unit-tested against Random(42) vectors) and every staged Java
    // call continues the SAME stream via the seed handoff.
    //
    // Kill-switch: -Dcentities.mobAi=0, any classification failure (unknown
    // goal class, oversized selector, triggered stroll) routes vanilla
    // forever for that mob; any engine ST_FALLBACK runs the byte-exact
    // serverAiStep mirror (the L15-floored GoalSelector bodies included).
    // ==================================================================

    // MEASURED NET LOSS on the 26.3 bench (e774d00: mobs-25k 193.2 vs
    // 129.5 without it): the staged-request protocol pays a pack+JNI round
    // per staged predicate, and the bench's TARGETED zombies stage a
    // canContinueToUse + canUse EVERY even tick — the rounds cost more than
    // the virtual calls they replace. Opt in with -Dcentities.mobAi=1
    // until the pre-resolved-predicate redesign lands (doc 30).
    public static final boolean MOB_AI_ON_INIT =
            "1".equals(System.getProperty("centities.mobAi", "0"));
    public static volatile boolean MOB_AI_ON = false;
    public static volatile boolean MOB_AI_HANDLES_OK = false;
    public static long mobAiTicks = 0, mobAiFallbacks = 0, mobAiRequests = 0,
            mobAiClassified = 0, mobAiRefused = 0, mobAiOddJava = 0;
    /** Diagnostics: reason (FB_*) + round + request op of the FIRST fallback
     *  this boot; the one-shot log line makes a classified-mob fallback
     *  diagnosable from the CI log alone (behavior itself stays exact — the
     *  mirror finishes that tick). */
    public static volatile int mobAiFallbackWhy = 0, mobAiFallbackRound = -1,
            mobAiFallbackReq = 0;
    public static volatile String mobAiFallbackMob = "-", mobAiRefusedClass = "-";
    private static volatile boolean mobAiFallbackLogged = false;
    private static volatile boolean mobAiRefusedLogged = false;

    // wire constants — MUST match centities-core/src/goals.rs
    static final int ST_DONE = 0, ST_REQUEST = 1, ST_FALLBACK = -1;
    static final int FB_BUF_TOO_SMALL = 1, FB_GOAL_OVERFLOW = 2,
            FB_LOCK_COLLIDE = 3, FB_UNKNOWN_KIND = 4, FB_BAD_CURSOR = 5,
            FB_UNKNOWN_REQ = 6, FB_ACTS_OVERFLOW = 7, FB_STATE_MISMATCH = 8,
            FB_JAVA_ROUND_CAP = 101, FB_JAVA_BAD_GI = 102,
            FB_JAVA_UNKNOWN_REQ = 103, FB_JAVA_PREDICATE = 104;
    static final int REQ_CC_VANILLA = 1, REQ_CANUSE_VANILLA = 2, REQ_STROLL_POS = 3,
            REQ_LOOKAT_QUERY = 4, REQ_START_VANILLA = 5, REQ_FIND_TARGET = 6;
    static final int GK_SPEAR_USE = 1, GK_ZOMBIE_ATTACK = 2, GK_MOVE_THROUGH_VILLAGE = 3,
            GK_STROLL = 4, GK_LOOKAT_PLAYER = 5, GK_LOOKAROUND = 6, GK_HURT_BY = 7,
            GK_NEAREST_TARGET = 8;
    static final int MF_NO_PASSENGERS = 1, MF_MAINHAND_EMPTY = 2, MF_HAS_TARGET = 4,
            MF_USING_ITEM = 8, MF_LAST_HURT_BY = 16, MF_BRIGHT_OUTSIDE = 32,
            MF_NEAR_VILLAGE = 64, MF_PEACEFUL = 128, MF_TEAMS_PRESENT = 256,
            MF_MANY_PLAYERS = 512;
    static final int ACT_GOAL_START = 1, ACT_GOAL_STOP = 2, ACT_GOAL_TICK_V = 3,
            ACT_LOOK_TICK_AROUND = 5, ACT_LOOK_TICK_AT = 6, ACT_REF_SET_PLAYER = 10;
    static final int REQ_LOS_SCAN = 7;
    static final int NAV_NONE = 0, NAV_PATH_DONE = 1, NAV_PATH_ACTIVE = 2;
    static final int MAX_GOALS = 16, GOAL_STRIDE = 8, IN_HEADER = 34, OUT_HEADER = 8;
    static final int IN_PCOUNT = 24, IN_FOLLOW = 25, IN_MOBX = 26, IN_MOBY = 28,
            IN_MOBZ = 30, IN_EYEY = 32;
    static final int PSTRIDE = 12, PX_LO = 0, PY_LO = 2, PZ_LO = 4, PEYE_LO = 6,
            PVIS_LO = 8, PFLAGS = 10, PID = 11;
    static final int AIPF_ALIVE = 1, AIPF_SPECTATOR = 2, AIPF_INVULNERABLE = 4,
            AIPF_INVISIBLE = 8, AIPF_DISCRETE = 16, AIPF_RIDING = 32;

    // per-mob classification context (weak identity map — no leaks, no
    // per-mob allocation after the first tick). MapMaker.makeMap() returns
    // its own MapMakerInternalMap — the field must be the ConcurrentMap
    // interface; a ConcurrentHashMap cast here KILLED the <clinit> and took
    // every gate down with it (the 40213481958 e2e failure).
    private static final java.util.concurrent.ConcurrentMap AI_CTX = buildAiCtxMap();

    private static java.util.concurrent.ConcurrentMap buildAiCtxMap() {
        try {
            return new com.google.common.collect.MapMaker()
                    .weakKeys().concurrencyLevel(1).makeMap();
        } catch (Throwable t) {
            // never let a classification-infra failure kill the <clinit> —
            // the non-weak fallback only leaks one AiCtx per classified mob
            // (bounded by the mob count) and keeps the module alive
            return new java.util.concurrent.ConcurrentHashMap<>();
        }
    }

    private static final class AiCtx {
        final Object targetSel, goalSel; // the selector objects (Unsafe-read)
        final net.minecraft.world.entity.ai.goal.WrappedGoal[] tGoals, gGoals;
        final int[] tKind, gKind, tPrio, gPrio;
        final long[] tFlags, gFlags;
        final int[] tCfg, tCfg3, gCfg, gCfg2, gCfg3; // cfg2: lookat range f32 bits; cfg3: flags
        final float[] lookDistSq; // per target-goal lookDistance (lookat kind)
        // MoveThroughVillage lazy-flag inputs: the village proximity read
        // (a POI-section probe) runs only when the mob HAS the goal AND the
        // night gate lets canUse proceed — otherwise it would burn the
        // vanilla canUse cost on every pack.
        final boolean hasMtv, mtvOnlyNight, fixedTime;
        AiCtx(Object ts, Object gs,
              net.minecraft.world.entity.ai.goal.WrappedGoal[] t,
              net.minecraft.world.entity.ai.goal.WrappedGoal[] g,
              int[] tk, int[] gk, int[] tp, int[] gp, long[] tf, long[] gf,
              int[] tc, int[] tc3, int[] gc, int[] gc2, int[] gc3, float[] lds,
              boolean hasMtv, boolean mtvOnlyNight, boolean fixedTime) {
            this.targetSel = ts; this.goalSel = gs;
            this.tGoals = t; this.gGoals = g;
            this.tKind = tk; this.gKind = gk;
            this.tPrio = tp; this.gPrio = gp;
            this.tFlags = tf; this.gFlags = gf;
            this.tCfg = tc; this.tCfg3 = tc3;
            this.gCfg = gc; this.gCfg2 = gc2; this.gCfg3 = gc3;
            this.lookDistSq = lds;
            this.hasMtv = hasMtv; this.mtvOnlyNight = mtvOnlyNight;
            this.fixedTime = fixedTime;
        }
        static final AiCtx REFUSED = new AiCtx(null, null,
                new net.minecraft.world.entity.ai.goal.WrappedGoal[0],
                new net.minecraft.world.entity.ai.goal.WrappedGoal[0],
                new int[0], new int[0], new int[0], new int[0],
                new long[0], new long[0], new int[0], new int[0], new int[0],
                new int[0], new int[0], new float[0], false, false, false);
    }

    // goal-object field offsets (resolved once per class at init)
    private static long O_RLA_LOOKTIME = -1, O_RLA_RELX = -1, O_RLA_RELZ = -1;
    private static long O_LAL_LOOKTIME = -1, O_LAL_LOOKAT = -1;
    private static long O_ZA_LASTCANUSE = -1;  // MeleeAttackGoal.lastCanUseCheck (long)
    private static long O_HB_TIMESTAMP = -1;   // HurtByTargetGoal.timestamp (int)
    private static long O_MTV_VISITED = -1;    // MoveThroughVillageGoal.visited (ArrayList)
    private static long O_MTV_ONLYNIGHT = -1;  // MoveThroughVillageGoal.onlyAtNight (boolean)
    private static long O_MOB_LASTHURTBY = -1, O_MOB_LASTHURTBY_TS = -1;
    private static long O_LVL_SKYDARKEN = -1;
    private static long O_NAT_TARGET = -1; // NearestAttackableTargetGoal.target
    private static long O_STROLL_FORCE = -1, O_STROLL_WX = -1, O_STROLL_WY = -1, O_STROLL_WZ = -1;
    private static long O_NAV_PATH = -1, O_NAV_TICK = -1, O_NAV_DELAYED = -1;
    private static long O_PATH_NEXT = -1, O_PATH_NODES = -1;
    private static long O_MC_OP = -1;
    private static long O_LE_ZZA = -1;
    private static long O_LC_COOL = -1, O_LC_WX = -1, O_LC_WY = -1, O_LC_WZ = -1,
            O_LC_YMAX = -1, O_LC_XMAX = -1;
    private static long O_JC_JUMP = -1, O_LE_JUMPING = -1;
    private static long O_MOB_TARGET = -1, O_MOB_NOACTION = -1, O_MOB_SENSE = -1;
    private static long O_MOB_TARGETSEL = -1, O_MOB_GOALSEL = -1;
    private static long O_ENT_POS = -1, O_ENT_XROT = -1, O_LIV_HEALTH = -1;
    private static long O_V3_X = -1, O_V3_Y = -1, O_V3_Z = -1;
    private static Object SHARED_RANDOM; // Entity.SHARED_RANDOM (static field ref)
    private static long O_RNG_VALUE = -1;
    private static java.lang.invoke.MethodHandle MH_GET_POSITION;   // RandomStrollGoal subclass
    private static java.lang.invoke.MethodHandle MH_STROLL_WX, MH_STROLL_WY, MH_STROLL_WZ;
    private static java.lang.invoke.MethodHandle MH_FIND_TARGET, MH_NEAREST_TARGET_F;
    private static java.lang.invoke.MethodHandle MH_LOOKAT_CTX;     // lookAtContext getter
    private static Object MOVE_OP_WAIT; // MoveControl.Operation.WAIT enum ref

    // scratch buffers (server-thread confined — one mob at a time)
    private static int[] AI_IN = new int[IN_HEADER + 2 * MAX_GOALS * GOAL_STRIDE];
    private static int[] AI_OUT = new int[OUT_HEADER + 96 * 8];

    /** One-time handle/offset resolution. Called via prove_handles at
     *  precompute time; sets MOB_AI_HANDLES_OK. */
    public static void initMobAiHandles() {
        try {
            Class<?> rla = Class.forName("net.minecraft.world.entity.ai.goal.RandomLookAroundGoal");
            O_RLA_LOOKTIME = offOf(rla, "lookTime");
            O_RLA_RELX = offOf(rla, "relX");
            O_RLA_RELZ = offOf(rla, "relZ");
            Class<?> lal = Class.forName("net.minecraft.world.entity.ai.goal.LookAtPlayerGoal");
            O_LAL_LOOKTIME = offOf(lal, "lookTime");
            O_LAL_LOOKAT = offOf(lal, "lookAt");
            Class<?> stroll = Class.forName("net.minecraft.world.entity.ai.goal.RandomStrollGoal");
            O_STROLL_FORCE = offOf(stroll, "forceTrigger");
            O_STROLL_WX = offOf(stroll, "wantedX");
            O_STROLL_WY = offOf(stroll, "wantedY");
            O_STROLL_WZ = offOf(stroll, "wantedZ");
            Class<?> nav = Class.forName("net.minecraft.world.entity.ai.navigation.PathNavigation");
            O_NAV_PATH = offOf(nav, "path");
            O_NAV_TICK = offOf(nav, "tick");
            O_NAV_DELAYED = offOf(nav, "hasDelayedRecomputation");
            Class<?> path = Class.forName("net.minecraft.world.level.pathfinder.Path");
            O_PATH_NEXT = offOf(path, "nextNodeIndex");
            O_PATH_NODES = offOf(path, "nodes");
            Class<?> mc = Class.forName("net.minecraft.world.entity.ai.control.MoveControl");
            O_MC_OP = offOf(mc, "operation");
            // L22b — canUse gate inputs for the three unconditional parks
            Class<?> za = Class.forName("net.minecraft.world.entity.ai.goal.MeleeAttackGoal");
            O_ZA_LASTCANUSE = offOf(za, "lastCanUseCheck");
            Class<?> hb = Class.forName("net.minecraft.world.entity.ai.goal.target.HurtByTargetGoal");
            O_HB_TIMESTAMP = offOf(hb, "timestamp");
            Class<?> mtv = Class.forName("net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal");
            O_MTV_VISITED = offOf(mtv, "visited");
            O_MTV_ONLYNIGHT = offOf(mtv, "onlyAtNight");
            // NOTE: no offOf on JDK classes (java.util.ArrayList.size) —
            // java.base is not open to the unnamed module, objectFieldOffset
            // throws InaccessibleObjectException; the visited size reads via
            // the virtual List.size() (monomorphic ArrayList) instead
            // 26.3: lastHurtByMob/lastHurtByMobTimestamp live on
            // LivingEntity (the former is an EntityReference — a non-null
            // reference is a SAFE SUPERSET for the HurtBy gate: the vanilla
            // canUse park re-resolves it, so a dangling reference only
            // costs a staged round, never a wrong decision)
            O_MOB_LASTHURTBY = offOf(net.minecraft.world.entity.LivingEntity.class, "lastHurtByMob");
            O_MOB_LASTHURTBY_TS = offOf(net.minecraft.world.entity.LivingEntity.class, "lastHurtByMobTimestamp");
            O_LVL_SKYDARKEN = offOf(net.minecraft.world.level.Level.class, "skyDarken");
            // v3 native scans: the goal reference fields the ACT_REF_SET_PLAYER
            // executor writes (vanilla getNearestPlayer/findTarget assign them)
            O_NAT_TARGET = offOf(Class.forName(
                    "net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal"),
                    "target");
            MOVE_OP_WAIT = java.lang.Enum.valueOf(
                    (Class<? extends java.lang.Enum>) Class.forName(
                            "net.minecraft.world.entity.ai.control.MoveControl$Operation"),
                    "WAIT");
            Class<?> lc = Class.forName("net.minecraft.world.entity.ai.control.LookControl");
            O_LC_COOL = offOf(lc, "lookAtCooldown");
            O_LC_WX = offOf(lc, "wantedX");
            O_LC_WY = offOf(lc, "wantedY");
            O_LC_WZ = offOf(lc, "wantedZ");
            O_LC_YMAX = offOf(lc, "yMaxRotSpeed");
            O_LC_XMAX = offOf(lc, "xMaxRotAngle");
            Class<?> jc = Class.forName("net.minecraft.world.entity.ai.control.JumpControl");
            O_JC_JUMP = offOf(jc, "jump");
            O_LE_JUMPING = offOf(net.minecraft.world.entity.LivingEntity.class, "jumping");
            O_LE_ZZA = offOf(net.minecraft.world.entity.LivingEntity.class, "zza");
            O_MOB_TARGET = offOf(net.minecraft.world.entity.Mob.class, "target");
            O_MOB_NOACTION = offOf(net.minecraft.world.entity.Mob.class, "noActionTime");
            O_MOB_TARGETSEL = offOf(net.minecraft.world.entity.Mob.class, "targetSelector");
            O_MOB_GOALSEL = offOf(net.minecraft.world.entity.Mob.class, "goalSelector");
            O_ENT_POS = offOf(net.minecraft.world.entity.Entity.class, "position");
            O_ENT_XROT = offOf(net.minecraft.world.entity.Entity.class, "xRot");
            Class<?> v3 = Class.forName("net.minecraft.world.phys.Vec3");
            O_V3_X = offOf(v3, "x");
            O_V3_Y = offOf(v3, "y");
            O_V3_Z = offOf(v3, "z");
            java.lang.reflect.Field sr = net.minecraft.world.entity.Entity.class
                    .getDeclaredField("SHARED_RANDOM");
            sr.setAccessible(true);
            SHARED_RANDOM = sr.get(null);
            O_RNG_VALUE = offOf(SHARED_RANDOM.getClass(), "value");
            // protected stroll override + goal-internal fields —
            // privateLookupIn: publicLookup CANNOT unreflect protected
            // members (the first bench run armed WITHOUT the engine:
            // "handle probe failed"), the unnamed module opens everything
            java.lang.invoke.MethodHandles.Lookup pub = java.lang.invoke.MethodHandles.lookup();
            try {
                Class<?> was = Class.forName(
                        "net.minecraft.world.entity.ai.goal.WaterAvoidingRandomStrollGoal");
                java.lang.invoke.MethodHandles.Lookup pl = java.lang.invoke.MethodHandles
                        .privateLookupIn(was, pub);
                MH_GET_POSITION = pl.findVirtual(was, "getPosition",
                        java.lang.invoke.MethodType.methodType(net.minecraft.world.phys.Vec3.class));
            } catch (Throwable notOverridden) {
                java.lang.invoke.MethodHandles.Lookup pl = java.lang.invoke.MethodHandles
                        .privateLookupIn(stroll, pub);
                MH_GET_POSITION = pl.findVirtual(stroll, "getPosition",
                        java.lang.invoke.MethodType.methodType(net.minecraft.world.phys.Vec3.class));
            }
            MH_STROLL_WX = java.lang.invoke.MethodHandles.privateLookupIn(stroll, pub)
                    .unreflectGetter(stroll.getDeclaredField("wantedX"));
            Class<?> nat = Class.forName(
                    "net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal");
            java.lang.invoke.MethodHandles.Lookup pnl = java.lang.invoke.MethodHandles
                    .privateLookupIn(nat, pub);
            MH_FIND_TARGET = pnl.findVirtual(nat, "findTarget",
                    java.lang.invoke.MethodType.methodType(void.class));
            MH_NEAREST_TARGET_F = pnl.unreflectGetter(nat.getDeclaredField("target"));
            Class<?> lalCls = Class.forName("net.minecraft.world.entity.ai.goal.LookAtPlayerGoal");
            MH_LOOKAT_CTX = java.lang.invoke.MethodHandles.privateLookupIn(lalCls, pub)
                    .unreflectGetter(lalCls.getDeclaredField("lookAtContext"));
            // L22c fix: the classification walks the mob's GoalSelectors —
            // the goalTypes/lockedFlags RefFields belong to THIS engine too.
            // They used to live only in the L15 initAIHandles; with the
            // GoalSelector floor shipped off (aiFloor=0) that init never ran
            // and a classification NPE'd mid-tick on a half-mutated mob (run
            // 40881390981). Resolve them HERE so the engine is
            // self-contained; the mobAiFloor entry guard keeps the
            // belt-and-suspenders fail-fast.
            VH_GS_DISABLED = new RefField(
                    net.minecraft.world.entity.ai.goal.GoalSelector.class, "goalTypes");
            VH_GS_LOCKED = new RefField(
                    net.minecraft.world.entity.ai.goal.GoalSelector.class, "lockedFlags");
            AI_HANDLES_SCRATCH_OK();
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initMobAiHandles: " + t;
            MOB_AI_HANDLES_OK = false;
        }
    }

    private static void AI_HANDLES_SCRATCH_OK() {
        if (SHARED_RANDOM == null || O_RNG_VALUE < 0 || O_MC_OP < 0 || O_NAV_PATH < 0
                || O_LC_COOL < 0 || O_JC_JUMP < 0 || O_MOB_TARGET < 0 || O_MOB_NOACTION < 0
                || O_ZA_LASTCANUSE < 0 || O_HB_TIMESTAMP < 0 || O_MTV_VISITED < 0
                || O_MTV_ONLYNIGHT < 0 || O_MOB_LASTHURTBY < 0
                || O_MOB_LASTHURTBY_TS < 0 || O_LVL_SKYDARKEN < 0 || O_NAT_TARGET < 0
                || O_RLA_LOOKTIME < 0 || O_LAL_LOOKAT < 0 || O_STROLL_FORCE < 0
                || O_ENT_POS < 0 || MH_GET_POSITION == null || MH_FIND_TARGET == null
                || MH_LOOKAT_CTX == null || MH_NEAREST_TARGET_F == null) {
            MOB_AI_HANDLES_OK = false;
        } else {
            MOB_AI_HANDLES_OK = true;
        }
    }

    /** The lookat alive/range checks use virtual isAlive() — no health
     *  offset needed (the SynchedEntityData-backed float would need the
     *  full accessor dance for one boolean). */

    /** Kind id for a goal class, or 0 (unknown → refuse). */
    private static int aiKindOf(String cn) {
        switch (cn) {
            case "net.minecraft.world.entity.ai.goal.SpearUseGoal": return GK_SPEAR_USE;
            case "net.minecraft.world.entity.ai.goal.ZombieAttackGoal":
            case "net.minecraft.world.entity.monster.zombie.Zombie$ZombieAttackTurtleEggGoal":
                return GK_ZOMBIE_ATTACK;
            case "net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal":
                return GK_MOVE_THROUGH_VILLAGE;
            case "net.minecraft.world.entity.ai.goal.WaterAvoidingRandomStrollGoal":
            case "net.minecraft.world.entity.ai.goal.RandomStrollGoal":
                return GK_STROLL;
            case "net.minecraft.world.entity.ai.goal.LookAtPlayerGoal":
                return GK_LOOKAT_PLAYER;
            case "net.minecraft.world.entity.ai.goal.RandomLookAroundGoal":
                return GK_LOOKAROUND;
            case "net.minecraft.world.entity.ai.goal.target.HurtByTargetGoal":
                return GK_HURT_BY;
            case "net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal":
                return GK_NEAREST_TARGET;
            default: return 0;
        }
    }

    private static int readIntField(Object o, Class<?> start, String name) {
        try {
            Class<?> k = start;
            while (k != null) {
                try {
                    java.lang.reflect.Field f = k.getDeclaredField(name);
                    f.setAccessible(true);
                    return f.getInt(o);
                } catch (NoSuchFieldException e) {
                    k = k.getSuperclass();
                }
            }
        } catch (Throwable ignore) {
        }
        return -1;
    }

    private static float readFloatField(Object o, Class<?> start, String name) {
        try {
            Class<?> k = start;
            while (k != null) {
                try {
                    java.lang.reflect.Field f = k.getDeclaredField(name);
                    f.setAccessible(true);
                    return f.getFloat(o);
                } catch (NoSuchFieldException e) {
                    k = k.getSuperclass();
                }
            }
        } catch (Throwable ignore) {
        }
        return -1.0F;
    }

    private static Object readObjectField(Object o, Class<?> start, String name) {
        try {
            Class<?> k = start;
            while (k != null) {
                try {
                    java.lang.reflect.Field f = k.getDeclaredField(name);
                    f.setAccessible(true);
                    return f.get(o);
                } catch (NoSuchFieldException e) {
                    k = k.getSuperclass();
                }
            }
        } catch (Throwable ignore) {
        }
        return null;
    }

    /** Classify a mob: exact Zombie class + the full known goal set in both
     *  selectors. Returns null when the mob must stay vanilla. */
    private static AiCtx aiClassify(net.minecraft.world.entity.Mob mob) {
        Object k = mob;
        Object hit = AI_CTX.get(k);
        if (hit != null) {
            return hit == AiCtx.REFUSED ? null : (AiCtx) hit;
        }
        AiCtx built = null;
        try {
            if (mob.getClass() != net.minecraft.world.entity.monster.zombie.Zombie.class) {
                return refuse(k);
            }
            java.util.Set<net.minecraft.world.entity.ai.goal.WrappedGoal> tSet =
                    ((net.minecraft.world.entity.ai.goal.GoalSelector) U.getObject(
                            mob, O_MOB_TARGETSEL)).getAvailableGoals();
            java.util.Set<net.minecraft.world.entity.ai.goal.WrappedGoal> gSet =
                    ((net.minecraft.world.entity.ai.goal.GoalSelector) U.getObject(
                            mob, O_MOB_GOALSEL)).getAvailableGoals();
            int nt = tSet.size(), ng = gSet.size();
            if (nt == 0 || nt > MAX_GOALS || ng == 0 || ng > MAX_GOALS) {
                return refuse(k);
            }
            net.minecraft.world.entity.ai.goal.WrappedGoal[] tg =
                    new net.minecraft.world.entity.ai.goal.WrappedGoal[nt];
            net.minecraft.world.entity.ai.goal.WrappedGoal[] gg =
                    new net.minecraft.world.entity.ai.goal.WrappedGoal[ng];
            int[] tk = new int[nt], tp = new int[nt];
            long[] tf = new long[nt];
            int[] tc = new int[nt];
            int[] tc3 = new int[nt];
            float[] lds = new float[nt];
            int i = 0;
            for (net.minecraft.world.entity.ai.goal.WrappedGoal wg : tSet) {
                tg[i] = wg;
                tk[i] = aiKindOf(wg.getGoal().getClass().getName());
                if (tk[i] == 0) return refuse(k);
                tp[i] = wg.getPriority();
                tf[i] = wg.getFlags().getBackingSet();
                if ((tf[i] & ~(1L | 2L | 4L | 8L | 16L)) != 0) return refuse(k); // unknown flags
                if (tk[i] == GK_NEAREST_TARGET) {
                    tc[i] = readIntField(wg.getGoal(), wg.getGoal().getClass(), "randomInterval");
                    // cfg bit1 = targetType == Player (native snapshot scan)
                    // cfg3 bit0 = Player-typed (native snapshot scan gate);
                    // cfg stays the pure randomInterval (bit1 collided with 10!)
                    tc3[i] = readObjectField(wg.getGoal(), wg.getGoal().getClass(),
                            "targetType") == net.minecraft.world.entity.player.Player.class
                            ? 1 : 0;
                } else if (tk[i] == GK_HURT_BY) {
                    tc[i] = 0;
                } else {
                    return refuse(k);
                }
                lds[i] = 0.0F;
                i++;
            }
            int[] gk = new int[ng], gp = new int[ng];
            long[] gf = new long[ng];
            int[] gc = new int[ng];
            int[] gc2 = new int[ng];
            int[] gc3 = new int[ng];
            boolean hasMtv = false, mtvOnlyNight = false;
            i = 0;
            for (net.minecraft.world.entity.ai.goal.WrappedGoal wg : gSet) {
                gg[i] = wg;
                gk[i] = aiKindOf(wg.getGoal().getClass().getName());
                if (gk[i] == 0) return refuse(k);
                gp[i] = wg.getPriority();
                gf[i] = wg.getFlags().getBackingSet();
                if ((gf[i] & ~(1L | 2L | 4L | 8L | 16L)) != 0) return refuse(k);
                Object goal = wg.getGoal();
                switch (gk[i]) {
                    case GK_STROLL:
                        int interval = readIntField(goal, goal.getClass(), "interval");
                        if (interval <= 0) return refuse(k);
                        if (!readBoolField(goal, goal.getClass(), "checkNoActionTime")) {
                            return refuse(k); // the engine assumes the check is on
                        }
                        // reducedTickDelay(interval) = positiveCeilDiv(interval, 2)
                        gc[i] = (interval + 1) / 2;
                        if (U.getByte(goal, O_STROLL_FORCE) != 0) return refuse(k);
                        break;
                    case GK_LOOKAT_PLAYER:
                        float prob = readFloatField(goal, goal.getClass(), "probability");
                        gc[i] = Float.floatToIntBits(prob);
                        lds[i] = readFloatField(goal, goal.getClass(), "lookDistance");
                        if (readObjectField(goal, goal.getClass(), "lookAtType")
                                == net.minecraft.world.entity.player.Player.class) {
                            gc2[i] = Float.floatToIntBits(lds[i]); // cfg2 = the context range
                            gc3[i] = 1; // cfg3 bit0 = Player-typed scan
                        }
                        break;
                    case GK_MOVE_THROUGH_VILLAGE:
                        hasMtv = true;
                        mtvOnlyNight = U.getByte(goal, O_MTV_ONLYNIGHT) != 0;
                        gc[i] = mtvOnlyNight ? 1 : 0;
                        break;
                    default:
                        gc[i] = 0;
                }
                i++;
            }
            boolean fixedTime = mob.level().dimensionType().hasFixedTime();
            built = new AiCtx(U.getObject(mob, O_MOB_TARGETSEL),
                    U.getObject(mob, O_MOB_GOALSEL),
                    tg, gg, tk, gk, tp, gp, tf, gf, tc, tc3, gc, gc2, gc3, lds,
                    hasMtv, mtvOnlyNight, fixedTime);
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "aiClassify: " + t;
            return refuse(k);
        }
        AI_CTX.put(k, built);
        mobAiClassified++;
        return built;
    }

    private static AiCtx refuse(Object k) {
        AI_CTX.put(k, AiCtx.REFUSED);
        mobAiRefused++;
        if (!mobAiRefusedLogged && k instanceof net.minecraft.world.entity.Mob m) {
            mobAiRefusedLogged = true;
            mobAiRefusedClass = m.getClass().getName();
            System.out.println("[c-entities] mob AI first refused class: " + mobAiRefusedClass);
        }
        return null;
    }

    private static boolean readBoolField(Object o, Class<?> start, String name) {
        try {
            Class<?> k = start;
            while (k != null) {
                try {
                    java.lang.reflect.Field f = k.getDeclaredField(name);
                    f.setAccessible(true);
                    return f.getBoolean(o);
                } catch (NoSuchFieldException e) {
                    k = k.getSuperclass();
                }
            }
        } catch (Throwable ignore) {
        }
        return false;
    }

    // sentinel: classification refused this mob (zero-length context)

    // ---- the packed-buffer engine driver ----

    private static void aiPack(net.minecraft.world.entity.Mob mob, AiCtx ctx,
                               int cursor, int res, int lastReqOp,
                               double rx, double ry, double rz) {
        int[] in = AI_IN;
        in[0] = cursor;
        int mf = 0;
        try {
            if (mob.hasControllingPassenger()) mf |= 0; else mf |= 1; // MF_NO_PASSENGERS=1
        } catch (Throwable t) {
            mf &= ~1;
        }
        if (mob.getMainHandItem().isEmpty()) mf |= 2;            // MF_MAINHAND_EMPTY=2
        if (U.getObject(mob, O_MOB_TARGET) != null) mf |= 4;     // MF_HAS_TARGET=4
        if (mob.isUsingItem()) mf |= 8;                          // MF_USING_ITEM=8
        if (U.getObject(mob, O_MOB_LASTHURTBY) != null) mf |= 16; // MF_LAST_HURT_BY=16
        in[1] = mf;
        in[2] = mob.tickCount;
        in[3] = mob.getId();
        // noActionTime already incremented by the floor (vanilla order)
        in[4] = U.getInt(mob, O_MOB_NOACTION);
        in[5] = ctx.tGoals.length;
        in[6] = ctx.gGoals.length;
        long dT = disabledBacking((net.minecraft.world.entity.ai.goal.GoalSelector) ctx.targetSel);
        long dG = disabledBacking((net.minecraft.world.entity.ai.goal.GoalSelector) ctx.goalSel);
        in[7] = (int) dT; in[8] = (int) (dT >>> 32);
        in[9] = (int) dG; in[10] = (int) (dG >>> 32);
        in[11] = aiNavState(mob);
        long seed = U.getLong(SHARED_RANDOM, O_RNG_VALUE);
        in[12] = (int) seed; in[13] = (int) (seed >>> 32);
        in[14] = res; in[15] = cursor != 0 ? lastReqOp : 0; // the resume dispatch op
        in[16] = (int) Double.doubleToRawLongBits(rx);
        in[17] = (int) (Double.doubleToRawLongBits(rx) >>> 32);
        in[18] = (int) Double.doubleToRawLongBits(ry);
        in[19] = (int) (Double.doubleToRawLongBits(ry) >>> 32);
        in[20] = (int) Double.doubleToRawLongBits(rz);
        in[21] = (int) (Double.doubleToRawLongBits(rz) >>> 32);
        // L22b — HurtBy gate timestamp + the level game time (MeleeAttack
        // 20-tick throttle; u32-wrapping compares in the engine)
        in[22] = U.getInt(mob, O_MOB_LASTHURTBY_TS);
        net.minecraft.server.level.ServerLevel lvl =
                (net.minecraft.server.level.ServerLevel) mob.level();
        in[23] = (int) lvl.getGameTime();
        // MoveThroughVillage lazy flags: the POI proximity probe runs only
        // when the mob HAS the goal AND the night gate lets canUse proceed.
        // vanilla: if (onlyAtNight && level.isBrightOutside()) return false;
        // isBrightOutside() = !hasFixedTime() && skyDarken < 4.
        if (ctx.hasMtv) {
            boolean bright = !ctx.fixedTime && U.getInt(lvl, O_LVL_SKYDARKEN) < 4;
            if (bright) mf |= MF_BRIGHT_OUTSIDE;
            if (!(ctx.mtvOnlyNight && bright)
                    && lvl.isCloseToVillage(mob.blockPosition(), 6)) {
                mf |= MF_NEAR_VILLAGE;
            }
            in[1] = mf; // publish the late flags
        }
        // v3 native scan inputs: level-global flags + the mob anchor + the
        // player snapshot (level.players() order, LIVE coords per pack)
        java.util.List<?> players = lvl.players();
        int p = players.size();
        mf |= MF_PEACEFUL;
        if (lvl.getDifficulty() != net.minecraft.world.Difficulty.PEACEFUL) mf &= ~MF_PEACEFUL;
        if (aiTeamsPresent(lvl)) mf |= MF_TEAMS_PRESENT;
        if (p > 30) mf |= MF_MANY_PLAYERS;
        in[1] = mf;
        in[IN_PCOUNT] = p;
        // TargetGoal.getFollowDistance() = getAttributeValue(FOLLOW_RANGE)
        in[IN_FOLLOW] = Float.floatToIntBits((float) mob.getAttributeValue(
                net.minecraft.world.entity.ai.attributes.Attributes.FOLLOW_RANGE));
        long bx = Double.doubleToRawLongBits(mob.getX());
        in[IN_MOBX] = (int) bx; in[IN_MOBX + 1] = (int) (bx >>> 32);
        long by = Double.doubleToRawLongBits(mob.getY());
        in[IN_MOBY] = (int) by; in[IN_MOBY + 1] = (int) (by >>> 32);
        long bz = Double.doubleToRawLongBits(mob.getZ());
        in[IN_MOBZ] = (int) bz; in[IN_MOBZ + 1] = (int) (bz >>> 32);
        long beye = Double.doubleToRawLongBits(mob.getEyeY());
        in[IN_EYEY] = (int) beye; in[IN_EYEY + 1] = (int) (beye >>> 32);
        if (p > 0) {
            aiRefreshPlayers(lvl, players, p);
            int need = IN_HEADER + (ctx.tGoals.length + ctx.gGoals.length) * GOAL_STRIDE
                    + p * PSTRIDE;
            if (AI_IN.length < need) {
                AI_IN = new int[Math.max(need, 2 * AI_IN.length)];
            }
            int[] in2 = AI_IN;
            int pb = IN_HEADER + (ctx.tGoals.length + ctx.gGoals.length) * GOAL_STRIDE;
            for (int i = 0; i < p; i++) {
                net.minecraft.world.entity.player.Player pl =
                        (net.minecraft.world.entity.player.Player) players.get(i);
                int b = pb + i * PSTRIDE;
                long lx = Double.doubleToRawLongBits(pl.getX());
                in2[b + PX_LO] = (int) lx; in2[b + PX_LO + 1] = (int) (lx >>> 32);
                long ly = Double.doubleToRawLongBits(pl.getY());
                in2[b + PY_LO] = (int) ly; in2[b + PY_LO + 1] = (int) (ly >>> 32);
                long lz = Double.doubleToRawLongBits(pl.getZ());
                in2[b + PZ_LO] = (int) lz; in2[b + PZ_LO + 1] = (int) (lz >>> 32);
                long ley = Double.doubleToRawLongBits(pl.getEyeY());
                in2[b + PEYE_LO] = (int) ley; in2[b + PEYE_LO + 1] = (int) (ley >>> 32);
                long lvis = Double.doubleToRawLongBits(aiVisPct(pl, lvl, mob));
                in2[b + PVIS_LO] = (int) lvis; in2[b + PVIS_LO + 1] = (int) (lvis >>> 32);
                int fl = 0;
                if (pl.isAlive()) fl |= AIPF_ALIVE;
                if (pl.isSpectator()) fl |= AIPF_SPECTATOR;
                if (pl.isInvulnerable()) fl |= AIPF_INVULNERABLE;
                if (pl.isInvisible()) fl |= AIPF_INVISIBLE;
                if (pl.isDiscrete()) fl |= AIPF_DISCRETE;
                if (pl.isPassenger()) fl |= AIPF_RIDING;
                in2[b + PFLAGS] = fl;
                in2[b + PID] = pl.getId();
            }
        }
        int base = IN_HEADER;
        base = aiPackSel(in, base, ctx.tGoals, ctx.tKind, ctx.tPrio, ctx.tFlags, ctx.tCfg,
                null, ctx.tCfg3, ctx.lookDistSq, mob);
        aiPackSel(in, base, ctx.gGoals, ctx.gKind, ctx.gPrio, ctx.gFlags, ctx.gCfg,
                ctx.gCfg2, ctx.gCfg3, ctx.lookDistSq, mob);
    }

    private static int aiPackSel(int[] in, int base,
            net.minecraft.world.entity.ai.goal.WrappedGoal[] goals, int[] kinds,
            int[] prios, long[] flags, int[] cfgs, int[] cfg2s, int[] cfg3s,
            float[] lookDistSq,
            net.minecraft.world.entity.Mob mob) {
        for (int i = 0; i < goals.length; i++) {
            int b = base + i * GOAL_STRIDE;
            in[b] = kinds[i];
            in[b + 1] = prios[i];
            in[b + 2] = (int) flags[i];
            in[b + 3] = cfgs[i];
            in[b + 4] = goals[i].isRunning() ? 1 : 0;
            int st = 0;
            Object goal = goals[i].getGoal();
            switch (kinds[i]) {
                case GK_LOOKAROUND:
                    st = U.getInt(goal, O_RLA_LOOKTIME);
                    break;
                case GK_ZOMBIE_ATTACK:
                    st = (int) U.getLong(goal, O_ZA_LASTCANUSE); // low 32 — u32-wrapping throttle
                    break;
                case GK_HURT_BY:
                    st = U.getInt(goal, O_HB_TIMESTAMP);
                    break;
                case GK_MOVE_THROUGH_VILLAGE: {
                    Object visited = U.getObject(goal, O_MTV_VISITED);
                    // the virtual List.size() (monomorphic ArrayList — the
                    // goal constructs Lists.newArrayList()); a null/exotic
                    // shape parks (state > 15 routes vanilla — always exact)
                    st = visited instanceof java.util.List<?> l ? l.size() : 999;
                    break;
                }
                case GK_LOOKAT_PLAYER: {
                    st = U.getInt(goal, O_LAL_LOOKTIME) & 0x3FFFFFFF;
                    Object lookAt = U.getObject(goal, O_LAL_LOOKAT);
                    if (lookAt instanceof net.minecraft.world.entity.LivingEntity la
                            && la.isAlive()) {
                        st |= 1 << 31;
                        double d2 = ctx_distSq(mob, la);
                        if (lookDistSq != null && lookDistSq[i] > 0.0F
                                && !(d2 > (double) lookDistSq[i] * lookDistSq[i])) {
                            st |= 1 << 30;
                        }
                    }
                    break;
                }
                default:
                    st = 0;
            }
            in[b + 5] = st;
            in[b + 6] = cfg2s != null ? cfg2s[i] : 0;
            in[b + 7] = cfg3s != null ? cfg3s[i] : 0;
        }
        return base + goals.length * GOAL_STRIDE;
    }

    private static double ctx_distSq(net.minecraft.world.entity.Mob mob,
                                    net.minecraft.world.entity.LivingEntity other) {
        // Entity.distanceToSqr(Entity): dx*dx + dy*dy + dz*dz in doubles —
        // the comparison must stay double (the vanilla cc compares doubles)
        double mx = mob.getX(), my = mob.getY(), mz = mob.getZ();
        double ox = other.getX(), oy = other.getY(), oz = other.getZ();
        double dx = mx - ox, dy = my - oy, dz = mz - oz;
        return dx * dx + dy * dy + dz * dz;
    }

    private static int aiNavState(net.minecraft.world.entity.Mob mob) {
        Object nav = mob.getNavigation();
        Object path = U.getObject(nav, O_NAV_PATH);
        if (path == null) return NAV_NONE;
        int next = U.getInt(path, O_PATH_NEXT);
        Object nodes = U.getObject(path, O_PATH_NODES);
        int n = nodes instanceof java.util.List<?> l ? l.size() : 0;
        return next >= n ? NAV_PATH_DONE : NAV_PATH_ACTIVE;
    }

    // ---- v3 native-scan Java side ----
    // The player snapshot array mirrors the packed order (level.players());
    // rebuilt once per (level, gameTime). Identities are stable within a
    // tick (joins/leaves process between ticks); COORDS are re-read per
    // pack (a player can move mid-tick — vanilla reads them live too).
    private static net.minecraft.world.entity.player.Player[] AI_PLAYERS =
            new net.minecraft.world.entity.player.Player[8];
    private static int AI_PLAYERS_N = 0;
    private static long AI_PLAYERS_TICK = -1;
    private static Object AI_PLAYERS_LEVEL = null;

    private static void aiRefreshPlayers(net.minecraft.server.level.ServerLevel lvl,
            java.util.List<?> players, int p) {
        long t = lvl.getGameTime();
        if (AI_PLAYERS_LEVEL == lvl && AI_PLAYERS_TICK == t && AI_PLAYERS_N == p) return;
        if (AI_PLAYERS.length < p) {
            AI_PLAYERS = new net.minecraft.world.entity.player.Player[Math.max(p, 8)];
        }
        for (int i = 0; i < p; i++) {
            AI_PLAYERS[i] = (net.minecraft.world.entity.player.Player) players.get(i);
        }
        AI_PLAYERS_N = p;
        AI_PLAYERS_TICK = t;
        AI_PLAYERS_LEVEL = lvl;
    }

    private static boolean AI_TEAMS_PRESENT = false;
    private static long AI_TEAMS_TICK = -1;

    /** Any scoreboard team activity disables the native target scans (the
     *  isAlliedTo half is not snapshot-computable); memoized per tick. */
    private static boolean aiTeamsPresent(net.minecraft.server.level.ServerLevel lvl) {
        long t = lvl.getGameTime();
        if (AI_TEAMS_TICK == t) return AI_TEAMS_PRESENT;
        boolean any = false;
        try {
            for (Object team : lvl.getScoreboard().getPlayerTeams()) {
                any = true;
                break;
            }
        } catch (Throwable ignore) {
            any = true; // cannot prove absence — stay vanilla
        }
        AI_TEAMS_PRESENT = any;
        AI_TEAMS_TICK = t;
        return any;
    }

    // getVisibilityPercent memo — the gear/stealth inputs change only
    // between ticks (packets process before the level tick), the targeter
    // half is type-based (the classified zombie) — valid per (player, tick)
    private static Object VIS_MEMO_PLAYER = null;
    private static long VIS_MEMO_TICK = -1;
    private static double VIS_MEMO_VAL = 1.0;

    private static double aiVisPct(net.minecraft.world.entity.player.Player pl,
            net.minecraft.server.level.ServerLevel lvl,
            net.minecraft.world.entity.Mob targeter) {
        long t = lvl.getGameTime();
        if (VIS_MEMO_PLAYER == pl && VIS_MEMO_TICK == t) return VIS_MEMO_VAL;
        double v = pl.getVisibilityPercent(lvl, targeter);
        VIS_MEMO_PLAYER = pl;
        VIS_MEMO_TICK = t;
        VIS_MEMO_VAL = v;
        return v;
    }

    /** The floor body — the EarlyReturnGuard check for Mob.serverAiStep. */
    public static boolean mobAiFloor(net.minecraft.world.entity.Mob mob) {
        // VH_GS_DISABLED (GoalSelector.goalTypes) is resolved by the L15
        // initAIHandles — a DIFFERENT init than initMobAiHandles. With the
        // L15 GoalSelector floor shipped off (aiFloor=0) it stays null while
        // MOB_AI_HANDLES_OK is true: a classification would then throw
        // MID-TICK (after noActionTime++ and sensing.tick(), stepped=true)
        // and the mirror-tail recovery desynced the leg (run 40881390981:
        // push-triggered harness armed the engine via the parity.yml empty-
        // input trap, the NPE fired at t~158, PH diverged at t=161, n=2507
        // vs 2509 at T382). Fail-fast BEFORE any mutation instead: the
        // guard declines, vanilla serverAiStep runs, behavior stays exact.
        if (!ACTIVE || !MOB_AI_ON || !MOB_AI_HANDLES_OK || VH_GS_DISABLED == null) {
            return false;
        }
        boolean stepped = false;
        try {
            AiCtx ctx = aiClassify(mob);
            if (ctx == null) return false;
            // vanilla order: noActionTime++ then sensing.clear()
            U.putInt(mob, O_MOB_NOACTION, U.getInt(mob, O_MOB_NOACTION) + 1);
            mob.getSensing().tick();
            stepped = true;
            mobAiTicks++;
            // L22c — the odd branch of serverAiStep runs tickRunningGoals(false)
            // on both selectors, i.e. it ticks ONLY goals whose inner goal
            // overrides requiresUpdateEveryTick(): exactly {GK_ZOMBIE_ATTACK
            // (MeleeAttackGoal), GK_LOOKAROUND} for the classified set — the
            // target-selector kinds do not override it (javap-verified).
            // When no Melee goal is RUNNING the whole branch is a lookaround
            // walk: lookTime-- + setLookAt, no RNG, no engine round-trip —
            // run it in pure Java and skip the JNI entirely (half the ticks
            // of a peaceful zombie).
            boolean odd = ((mob.tickCount + mob.getId()) & 1) != 0 && mob.tickCount > 1;
            if (odd && !aiMeleeRunning(ctx)) {
                for (int i = 0; i < ctx.gGoals.length; i++) {
                    if (ctx.gKind[i] == GK_LOOKAROUND && ctx.gGoals[i].isRunning()) {
                        Object g = ctx.gGoals[i].getGoal();
                        actLookTickAround(mob, ctx.gGoals[i],
                                U.getInt(g, O_RLA_LOOKTIME) - 1);
                    }
                }
                aiTail(mob);
                mobAiOddJava++;
                return true;
            }
            int status = aiRun(mob, ctx);
            if (status == ST_FALLBACK) {
                mobAiFallbacks++;
                mobAiSelectorsMirror(mob);
                return true;
            }
            return true;
        } catch (Throwable t) {
            // a live floor failure disarms globally; when the step had
            // already mutated goal state, finish with the exact mirror
            // instead of re-running vanilla under a half-applied tick
            MOB_AI_ON = false;
            lastError = System.nanoTime();
            lastErrorText = "mobAiFloor: " + t;
            if (stepped) {
                try {
                    mobAiSelectorsMirror(mob);
                } catch (Throwable ignore) {
                }
                return true;
            }
            return false; // nothing mutated — vanilla runs the whole step
        }
    }

    /** True when a Melee-attack goal (the only every-tick kind besides
     *  RandomLookAroundGoal) is RUNNING — the odd-tick Java shortcut is
     *  exact only when it is not. */
    private static boolean aiMeleeRunning(AiCtx ctx) {
        for (int i = 0; i < ctx.gGoals.length; i++) {
            if (ctx.gKind[i] == GK_ZOMBIE_ATTACK && ctx.gGoals[i].isRunning()) return true;
        }
        return false;
    }

    private static int aiRun(net.minecraft.world.entity.Mob mob, AiCtx ctx) {
        int want = IN_HEADER + 2 * MAX_GOALS * GOAL_STRIDE
                + (AI_PLAYERS_N + 1) * PSTRIDE;
        if (AI_IN.length < want) {
            AI_IN = new int[Math.max(want, 2 * AI_IN.length)];
        }
        int cursor = 0, res = 0, lastReqOp = 0;
        double rx = 0, ry = 0, rz = 0;
        // 32 rounds: the cap is a run-away guard, not a semantic limit — a
        // legal tick can need one round per staged request (lookat scan,
        // findTarget, stroll pos, starts, ccs). 8 was demonstrably too low
        // (e2e: a burst tick fell back, the mirror covered it, but the
        // engine engagement was lost for that tick).
        for (int round = 0; round < 32; round++) {
            aiPack(mob, ctx, cursor, res, lastReqOp, rx, ry, rz);
            int status = nativeMobAiFloor(AI_IN, AI_OUT);
            if (status == ST_FALLBACK) {
                mobAiFallbackWhy = AI_OUT.length > 7 ? AI_OUT[7] : 0;
                mobAiFallbackRound = round;
                mobAiFallbackReq = lastReqOp;
                mobAiFallbackMob = mob.getClass().getName();
                logFirstFallback();
                return ST_FALLBACK;
            }
            // apply the actions IMMEDIATELY after every round — the engine
            // emits stop/start transitions in the exact vanilla order, and
            // later rounds' packs must observe their effects
            long seed = ((long) AI_OUT[4] << 32) | (AI_OUT[3] & 0xFFFF_FFFFL);
            U.putLong(SHARED_RANDOM, O_RNG_VALUE, seed);
            aiApply(mob, ctx);
            if (status == ST_DONE) {
                aiTail(mob);
                return ST_DONE;
            }
            // ST_REQUEST — resolve the PREDICATE only (no start/stop — the
            // engine owns the transitions); the seed was already applied
            // above so the predicate's RNG use continues the same stream
            mobAiRequests++;
            cursor = AI_OUT[6];
            int req = AI_OUT[1];
            lastReqOp = req;
            int gi = AI_OUT[2];
            res = 1;
            rx = ry = rz = 0;
            boolean targetSel = ((cursor >> 9) & 1) == 0;
            net.minecraft.world.entity.ai.goal.WrappedGoal[] arr =
                    targetSel ? ctx.tGoals : ctx.gGoals;
            if (gi < 0 || gi >= arr.length) {
                mobAiFallbackWhy = FB_JAVA_BAD_GI;
                mobAiFallbackRound = round;
                mobAiFallbackReq = lastReqOp;
                mobAiFallbackMob = mob.getClass().getName();
                logFirstFallback();
                return ST_FALLBACK;
            }
            net.minecraft.world.entity.ai.goal.WrappedGoal wg = arr[gi];
            switch (req) {
                case REQ_CC_VANILLA:
                    res = wg.canContinueToUse() ? 1 : 0;
                    break;
                case REQ_CANUSE_VANILLA:
                    res = wg.canUse() ? 1 : 0;
                    break;
                case REQ_START_VANILLA:
                    res = 1; // the engine emits the start action
                    break;
                case REQ_STROLL_POS: {
                    Object pos = null;
                    try {
                        pos = MH_GET_POSITION.invoke(wg.getGoal());
                    } catch (Throwable t) {
                        mobAiFallbackWhy = FB_JAVA_PREDICATE;
                        mobAiFallbackRound = round;
                        mobAiFallbackReq = lastReqOp;
                        mobAiFallbackMob = mob.getClass().getName();
                        logFirstFallback();
                        return ST_FALLBACK;
                    }
                    if (pos instanceof net.minecraft.world.phys.Vec3 v) {
                        Object g = wg.getGoal();
                        U.putDouble(g, O_STROLL_WX, v.x);
                        U.putDouble(g, O_STROLL_WY, v.y);
                        U.putDouble(g, O_STROLL_WZ, v.z);
                        U.putByte(g, O_STROLL_FORCE, (byte) 0);
                        rx = v.x; ry = v.y; rz = v.z;
                    } else {
                        res = 0;
                    }
                    break;
                }
                case REQ_LOOKAT_QUERY: {
                    net.minecraft.server.level.ServerLevel lvl =
                            (net.minecraft.server.level.ServerLevel) mob.level();
                    Object cond = null;
                    try {
                        cond = MH_LOOKAT_CTX.invoke(wg.getGoal());
                    } catch (Throwable t) {
                        mobAiFallbackWhy = FB_JAVA_PREDICATE;
                        mobAiFallbackRound = round;
                        mobAiFallbackReq = lastReqOp;
                        mobAiFallbackMob = mob.getClass().getName();
                        logFirstFallback();
                        return ST_FALLBACK;
                    }
                    net.minecraft.world.entity.player.Player p = lvl.getNearestPlayer(
                            (net.minecraft.world.entity.ai.targeting.TargetingConditions) cond,
                            mob, mob.getX(), mob.getEyeY(), mob.getZ());
                    if (p == null) res = 0;
                    break;
                }
                case REQ_FIND_TARGET: {
                    try {
                        MH_FIND_TARGET.invoke(wg.getGoal());
                        Object t = MH_NEAREST_TARGET_F.invoke(wg.getGoal());
                        res = t != null ? 1 : 0;
                    } catch (Throwable t2) {
                        mobAiFallbackWhy = FB_JAVA_PREDICATE;
                        mobAiFallbackRound = round;
                        mobAiFallbackReq = lastReqOp;
                        mobAiFallbackMob = mob.getClass().getName();
                        logFirstFallback();
                        return ST_FALLBACK;
                    }
                    break;
                }
                case REQ_LOS_SCAN: {
                    // the engine staged the non-LoS-passing candidates
                    // (snapshot indices, vanilla list order); raycast EVERY
                    // one through the Sensing cache (vanilla getNearestPlayer
                    // has no early exit) and resume with a bitmask over the
                    // SNAPSHOT indices
                    int cands = AI_OUT[8];
                    int mask = 0;
                    net.minecraft.world.entity.ai.sensing.Sensing sensing = mob.getSensing();
                    for (int c = 0; c < cands; c++) {
                        int pi = AI_OUT[9 + c];
                        if (pi < 0 || pi >= AI_PLAYERS_N) continue;
                        if (sensing.hasLineOfSight(AI_PLAYERS[pi])) {
                            mask |= (1 << pi);
                        }
                    }
                    res = mask;
                    break;
                }
                default:
                    mobAiFallbackWhy = FB_JAVA_UNKNOWN_REQ;
                    mobAiFallbackRound = round;
                    mobAiFallbackReq = lastReqOp;
                    mobAiFallbackMob = mob.getClass().getName();
                    logFirstFallback();
                    return ST_FALLBACK;
            }
        }
        mobAiFallbackWhy = FB_JAVA_ROUND_CAP;
        mobAiFallbackRound = 32;
        mobAiFallbackReq = lastReqOp;
        mobAiFallbackMob = mob.getClass().getName();
        logFirstFallback();
        return ST_FALLBACK; // round cap — the mirror finishes the tick
    }

    /** One-shot diagnostics: the first fallback of this boot names its
     *  reason/round/request/mob class on the server log. */
    private static void logFirstFallback() {
        if (mobAiFallbackLogged) return;
        mobAiFallbackLogged = true;
        System.out.println("[c-entities] mob AI first fallback: why=" + mobAiFallbackWhy
                + " round=" + mobAiFallbackRound + " req=" + mobAiFallbackReq
                + " mob=" + mobAiFallbackMob);
    }

    /** RandomLookAroundGoal.tick() byte-exact port — lookTime-- then
     *  setLookAt(x+relX, eyeY, z+relZ). relX/relZ are DOUBLE fields in the
     *  26.3 mappings (decompile-verified); the old getFloat read half the
     *  bytes and silently drifted the look direction. Shared by the action
     *  executor and the odd-tick Java walk. */
    private static void actLookTickAround(net.minecraft.world.entity.Mob mob,
            net.minecraft.world.entity.ai.goal.WrappedGoal wg, int newLookTime) {
        Object g = wg.getGoal();
        U.putInt(g, O_RLA_LOOKTIME, newLookTime);
        double relX = U.getDouble(g, O_RLA_RELX);
        double relZ = U.getDouble(g, O_RLA_RELZ);
        aiLookWrite(mob, mob.getX() + relX, mob.getEyeY(), mob.getZ() + relZ);
    }

    private static void aiApply(net.minecraft.world.entity.Mob mob, AiCtx ctx) {
        int n = AI_OUT[5];
        for (int a = 0; a < n; a++) {
            int b = OUT_HEADER + a * 8;
            int op = AI_OUT[b];
            int p0 = AI_OUT[b + 1];
            int p1 = AI_OUT[b + 2];
            switch (op) {
                case ACT_GOAL_START: {
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    wg.start();
                    break;
                }
                case ACT_GOAL_TICK_V: {
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    wg.tick();
                    break;
                }
                case ACT_GOAL_STOP: {
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    wg.stop();
                    break;
                }
                case ACT_LOOK_TICK_AROUND: {
                    // lookaround tick port: lookTime--, setLookAt(x+relX, eyeY, z+relZ)
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    actLookTickAround(mob, wg, p1);
                    break;
                }
                case ACT_REF_SET_PLAYER: {
                    // vanilla getNearestPlayer/findTarget assigns the goal's
                    // lookAt/target field; p1 < 0 writes null (the field is
                    // observable through start() and stale across stops for
                    // NearestAttackableTargetGoal which has no stop() clear)
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    Object pl = (p1 >= 0 && p1 < AI_PLAYERS_N) ? AI_PLAYERS[p1] : null;
                    Object g = wg.getGoal();
                    if (kindAt(ctx, p0) == GK_LOOKAT_PLAYER) {
                        U.putObject(g, O_LAL_LOOKAT, pl);
                    } else if (kindAt(ctx, p0) == GK_NEAREST_TARGET) {
                        U.putObject(g, O_NAT_TARGET, pl);
                    }
                    break;
                }
                case ACT_LOOK_TICK_AT: {
                    net.minecraft.world.entity.ai.goal.WrappedGoal wg = goalAt(ctx, p0);
                    if (wg == null) return;
                    Object g = wg.getGoal();
                    U.putInt(g, O_LAL_LOOKTIME, p1);
                    Object lookAt = U.getObject(g, O_LAL_LOOKAT);
                    if (lookAt instanceof net.minecraft.world.entity.Entity la) {
                        aiLookWrite(mob, la.getX(), la.getEyeY(), la.getZ());
                    }
                    break;
                }
                default:
                    return;
            }
        }
    }

    private static int kindAt(AiCtx ctx, int gi) {
        return gi < ctx.tKind.length ? ctx.tKind[gi] : ctx.gKind[gi - ctx.tKind.length];
    }

    private static net.minecraft.world.entity.ai.goal.WrappedGoal goalAt(AiCtx ctx, int gi) {
        // the engine emits GLOBAL indices: 0..nt-1 = target selector,
        // nt..nt+ng-1 = goal selector (matching the packed record order)
        int nt = ctx.tGoals.length;
        if (gi < 0) return null;
        if (gi < nt) return ctx.tGoals[gi];
        int g = gi - nt;
        if (g < ctx.gGoals.length) return ctx.gGoals[g];
        return null;
    }

    /** lookControl.setLookAt(x, y, z) equivalent — 6 field writes with the
     *  virtual speed getters the vanilla body calls. */
    private static void aiLookWrite(net.minecraft.world.entity.Mob mob,
                                    double x, double y, double z) {
        Object lc = mob.getLookControl();
        U.putDouble(lc, O_LC_WX, x);
        U.putDouble(lc, O_LC_WY, y);
        U.putDouble(lc, O_LC_WZ, z);
        U.putFloat(lc, O_LC_YMAX, mob.getHeadRotSpeed());
        U.putFloat(lc, O_LC_XMAX, mob.getMaxHeadXRot());
        U.putInt(lc, O_LC_COOL, 2);
    }

    /** The post-selector tail: navigation.tick (idle port), controls. */
    private static void aiTail(net.minecraft.world.entity.Mob mob) {
        // navigation.tick — vanilla: tick++; delayed recompute; if !isDone ...
        net.minecraft.world.entity.ai.navigation.PathNavigation nav = mob.getNavigation();
        boolean delayed = U.getByte(nav, O_NAV_DELAYED) != 0;
        int navState = aiNavState(mob);
        if (delayed || navState == NAV_PATH_ACTIVE) {
            nav.tick(); // the vanilla body (path following is a later cut)
        } else {
            U.putInt(nav, O_NAV_TICK, U.getInt(nav, O_NAV_TICK) + 1);
        }
        // moveControl.tick — WAIT branch: setZza(0)
        net.minecraft.world.entity.ai.control.MoveControl mc = mob.getMoveControl();
        if (U.getObject(mc, O_MC_OP) == MOVE_OP_WAIT) {
            U.putFloat(mob, O_LE_ZZA, 0.0F);
        } else {
            mc.tick();
        }
        // lookControl.tick — the cooldown branch routes vanilla (Mth.atan2)
        net.minecraft.world.entity.ai.control.LookControl lc = mob.getLookControl();
        if (U.getInt(lc, O_LC_COOL) > 0) {
            lc.tick();
        } else {
            U.putFloat(mob, O_ENT_XROT, 0.0F);
            float yHead = mob.yHeadRot;
            float rotated = aiRotateTowards(yHead, mob.yBodyRot, 10.0F);
            mob.yHeadRot = rotated;
            if (aiNavState(mob) == NAV_PATH_ACTIVE) {
                // !isDone → clampHeadRotationToBody
                float clamped = aiRotateIfNecessary(mob.yHeadRot, mob.yBodyRot,
                        mob.getMaxHeadYRot());
                mob.yHeadRot = clamped;
            }
        }
        // jumpControl.tick — setJumping(jump); jump = false
        net.minecraft.world.entity.ai.control.JumpControl jc = mob.getJumpControl();
        boolean jump = U.getByte(jc, O_JC_JUMP) != 0;
        U.putByte(mob, O_LE_JUMPING, (byte) (jump ? 1 : 0));
        U.putByte(jc, O_JC_JUMP, (byte) 0);
    }

    /** Control.rotateTowards — bit-exact f32 port. */
    private static float aiRotateTowards(float from, float to, float maxRot) {
        float diff = aiWrapDegrees(to - from);
        if (diff < -maxRot) diff = -maxRot;
        if (diff > maxRot) diff = maxRot;
        return from + diff;
    }

    /** Mth.rotateIfNecessary. */
    private static float aiRotateIfNecessary(float base, float target, float maxDiff) {
        float d = aiWrapDegrees(target - base);
        if (d < -maxDiff) d = -maxDiff;
        if (d > maxDiff) d = maxDiff;
        return target - d;
    }

    /** Mth.wrapDegrees(float). */
    private static float aiWrapDegrees(float a) {
        float n = a % 360.0F;
        if (n >= 180.0F) n -= 360.0F;
        if (n < -180.0F) n += 360.0F;
        return n;
    }

    /** The native goal engine (centities-core/src/goals.rs) — returns
     *  ST_DONE / ST_REQUEST / ST_FALLBACK and fills out. */
    public static native int nativeMobAiFloor(int[] in, int[] out);

    public static void setMobAiOn(boolean on) {
        MOB_AI_ON = on;
    }

    /** The byte-exact serverAiStep mirror for the post-noActionTime/sensing
     *  remainder (engine fallback + disarm path). The GoalSelector calls
     *  ride the L15 floor (itself a byte-exact vanilla sequence). The
     *  classified mob is exactly Zombie → customServerAiStep is the empty
     *  base body (verified in the decompile) — skipped. */
    private static void mobAiSelectorsMirror(net.minecraft.world.entity.Mob mob) {
        // the selectors ride the Unsafe-read refs (the fields are protected
        // in the 26.3 mapping — no public getters)
        net.minecraft.world.entity.ai.goal.GoalSelector ts =
                (net.minecraft.world.entity.ai.goal.GoalSelector) U.getObject(mob, O_MOB_TARGETSEL);
        net.minecraft.world.entity.ai.goal.GoalSelector gs =
                (net.minecraft.world.entity.ai.goal.GoalSelector) U.getObject(mob, O_MOB_GOALSEL);
        int idBasedTickCount = mob.tickCount + mob.getId();
        if (idBasedTickCount % 2 != 0 && mob.tickCount > 1) {
            ts.tickRunningGoals(false);
            gs.tickRunningGoals(false);
        } else {
            ts.tick();
            gs.tick();
        }
        mob.getNavigation().tick();
        mob.getMoveControl().tick();
        mob.getLookControl().tick();
        mob.getJumpControl().tick();
    }


    // ---- L20b: the same-value delta gate ----
    // Woven EARLY_RETURN_GUARD2 on Entity.setDeltaMovement(Vec3). The
    // quiet-mob travel tail rewrites the IDENTICAL delta every tick (the
    // (0, -g*vf, 0) fixed point) and every noGravity entity's applyGravity
    // allocates a value-equal Vec3 — vanilla's write is a value no-op but
    // the L12 probe mirror behind it costs ~40ns per call (worldId + the
    // batch append: probeDelta + worldId measured 4% of the mobs leg).
    // Skipping the whole call when the value is bit-identical is exact:
    // every deltaMovement reader is value-based EXCEPT Entity.move's
    // `delta != getDeltaMovement()` identity term, which is unreachable
    // for non-temporarily-active entities (the `!isTemporarilyActive`
    // disjunct short-circuits) — so those route vanilla below.
    public static volatile boolean DELTA_ON = false;
    public static final boolean DELTA_FLAG_ON =
            !"false".equals(System.getProperty("centities.deltaGate", "true"));
    public static long deltaCalls = 0, deltaGate = 0;

    public static void setDeltaGateOn(boolean on) {
        DELTA_ON = on;
    }

    public static boolean deltaNoOp(Entity e, net.minecraft.world.phys.Vec3 v) {
        deltaCalls++; // counts every woven call (the arming verifier's liveness proof)
        if (!DELTA_ON) return false;
        // vanilla silently ignores non-finite deltas — skipping is exact
        if (!v.isFinite()) return true;
        try {
            if (e.isTemporarilyActive) return false;
            net.minecraft.world.phys.Vec3 d = e.getDeltaMovement();
            if (d.x == v.x && d.y == v.y && d.z == v.z) {
                deltaGate++;
                return true;
            }
            return false;
        } catch (Throwable gateFail) {
            return false; // never lose a velocity write
        }
    }

    /** Replacement body of Entity.setSharedFlag(int, boolean) — byte-exact
     *  mirror with the equal-value early-out. */
    @SuppressWarnings("unchecked")
    public static void setSharedFlag$(Entity e, int flag, boolean set) {
        flagCalls++;
        if (FLAGG_ON && FLAGG_HANDLES_OK) {
            // L25: the direct-read gate — the byte comes off the DataItem
            // directly (one typed Unsafe walk), replacing the boxed get
            // chain. The computed-byte equality decision is the one
            // vanilla's SynchedEntityData.set would make (ObjectUtils
            // .notEqual on the boxed byte).
            try {
                // L29: the frozen-index read first; the accessor walk stays
                // as the structural fallback for a lean miss.
                Object cur = entityDataValueIdx(e, IDX_FLAGS);
                if (cur == null) {
                    cur = entityDataValue(e, FLAGS_ID);
                }
                if (cur instanceof Byte b0) {
                    int old = b0.byteValue();
                    int nw = set
                            ? (old | (1 << flag))
                            : (old & ~(1 << flag));
                    if ((byte) nw == old) {
                        flagGate++;
                        return;
                    }
                    flagWrites++;
                }
            } catch (Throwable gateFail) {
                // never swallow the write — fall through
            }
            try {
                net.minecraft.network.syncher.SynchedEntityData ed =
                        (net.minecraft.network.syncher.SynchedEntityData)
                                VH_ENT_DATA.getObj(e);
                net.minecraft.network.syncher.EntityDataAccessor<Byte> id =
                        (net.minecraft.network.syncher.EntityDataAccessor<Byte>) FLAGS_ID;
                byte old = ed.get(id);
                byte nw = set
                        ? (byte) (old | (1 << flag))
                        : (byte) (old & ~(1 << flag));
                if (nw == old) {
                    flagGate++;
                    return;
                }
                ed.set(id, Byte.valueOf(nw));
                flagWrites++;
                return;
            } catch (Throwable gateFail) {
                // never swallow the write — fall through to the vanilla mirror
            }
        }
        // vanilla mirror (byte-exact port of the 26.3 body). FLAGS_ID is
        // proven at weave time (the precompute refuses the bridge when
        // initFlagHandles failed); the lazy re-resolve covers a defensive
        // disarm — and a total failure rethrows into the caller rather than
        // silently dropping a flags write.
        try {
            net.minecraft.network.syncher.SynchedEntityData ed =
                    (net.minecraft.network.syncher.SynchedEntityData) VH_ENT_DATA.getObj(e);
            @SuppressWarnings("unchecked")
            net.minecraft.network.syncher.EntityDataAccessor<Byte> id =
                    (net.minecraft.network.syncher.EntityDataAccessor<Byte>) flagsId();
            byte b = ed.get(id);
            ed.set(id, set ? Byte.valueOf((byte) (b | (1 << flag)))
                           : Byte.valueOf((byte) (b & ~(1 << flag))));
        } catch (Throwable mirrorFail) {
            FLAGG_HANDLES_OK = false;
            lastError = System.nanoTime();
            lastErrorText = "flagMirror: " + mirrorFail;
            if (mirrorFail instanceof RuntimeException re) throw re;
            throw new IllegalStateException("setSharedFlag mirror failed", mirrorFail);
        }
    }

    /** The DATA_SHARED_FLAGS_ID accessor, re-resolved lazily if the Unsafe
     *  capture was unavailable (never on the hot path — the capture runs at
     *  handle-init and the weave is refused without it). */
    private static Object flagsId() throws Exception {
        Object f = FLAGS_ID;
        if (f == null) {
            java.lang.reflect.Field fd = Entity.class.getDeclaredField("DATA_SHARED_FLAGS_ID");
            fd.setAccessible(true);
            f = fd.get(null);
            FLAGS_ID = f;
        }
        return f;
    }

    public static void initConvHandles() {
        try {
            VH_CONV_MOB = new RefField(net.minecraft.world.entity.ConversionTracker.class, "mob");
            VH_CONV_ID = new RefField(
                    net.minecraft.world.entity.ConversionTracker.class, "conversionId");
            // L25: direct offsets for the fast gate (single-class on
            // ConversionTracker — the weave guarantees the receiver).
            if (U != null) {
                O_CT_MOB = U.objectFieldOffset(
                        net.minecraft.world.entity.ConversionTracker.class
                                .getDeclaredField("mob"));
                O_CT_ID = U.objectFieldOffset(
                        net.minecraft.world.entity.ConversionTracker.class
                                .getDeclaredField("conversionId"));
            }
            if (VH_ENT_DATA == null) {
                VH_ENT_DATA = new RefField(Entity.class, "entityData");
                initDataOffsets();
                if (VH_ENT_DATA.off >= 0) O_ED = VH_ENT_DATA.off;
            }
            CONV_HANDLES_OK = true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initConvHandles: " + t;
            CONV_HANDLES_OK = false;
        }
    }

    // ==================================================================
    // L15c — the tracker-tick no-op gate (doc 26 §6).
    //
    // ChunkMap$TrackedEntity.moonrise$tick(chunk) in a playerless world:
    // chunkData.nearbyPlayers is null for every chunk, so the whole call is
    // `moonrise$clearPlayers()` — two idempotent writes (lastChunkUpdate =
    // -1, lastTrackedChunk = null) + a seenBy.isEmpty() early return
    // (javap-verified). The items-100k-mod profile puts 14% of the leg's
    // CPU in that call chain (100k trackers × 20+ ticks/s), plus the
    // moonrise$tick entry frames around it. The gate returns TRUE only
    // when the tracker is ALREADY in the cleared state — the writes vanilla
    // would perform are provable no-ops — and NULL-chunk is the only shape
    // it skips: any non-null chunk (real players nearby) routes vanilla
    // wholesale. seenBy non-empty or stale bookkeeping fields route
    // vanilla too, so the clearPlayers side effects always run exactly
    // when vanilla would run them with a different result.
    // ==================================================================

    public static volatile boolean TT_ON = false;
    public static volatile boolean TT_HANDLES_OK = false;
    public static final boolean TT_FLAG_ON =
            !"false".equals(System.getProperty("centities.trackerTick", "true"));
    public static long ttCalls = 0, ttGate = 0;

    private static volatile RefField VH_TE_SEEN; // TrackedEntity.seenBy
    private static volatile RefField VH_TE_LCU;  // TrackedEntity.lastChunkUpdate
    private static volatile RefField VH_TE_LTC;  // TrackedEntity.lastTrackedChunk

    public static void initTTHandles() {
        try {
            Class<?> te = Class.forName("net.minecraft.server.level.ChunkMap$TrackedEntity");
            VH_TE_SEEN = new RefField(te, "seenBy");
            VH_TE_LCU = new RefField(te, "lastChunkUpdate");
            VH_TE_LTC = new RefField(te, "lastTrackedChunk");
            if (U != null && VH_TE_SEEN.off >= 0 && VH_TE_LCU.off >= 0
                    && VH_TE_LTC.off >= 0) {
                O_TE_SEEN = VH_TE_SEEN.off;
                O_TE_LCU = VH_TE_LCU.off;
                O_TE_LTC = VH_TE_LTC.off;
                TT_FAST = true;
            }
            TT_HANDLES_OK = true;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initTTHandles: " + t;
            TT_HANDLES_OK = false;
        }
    }

    private static long O_TE_SEEN, O_TE_LCU, O_TE_LTC;
    private static volatile boolean TT_FAST = false;

    /** moonrise$tick guard: (TrackedEntity this, NearbyPlayers$TrackedChunk chunk)Z.
     *  TRUE = the vanilla body is a provable no-op (skip it wholesale). */
    public static boolean trackerTickNoOp(Object te, Object chunk) {
        ttCalls++;
        if (!TT_ON) return false;
        if (TT_FAST) { // direct offsets: no megamorphic RefField dispatch
            if (chunk != null) return false;
            java.util.Set<?> seen = (java.util.Set<?>) U.getObject(te, O_TE_SEEN);
            if (seen == null || !seen.isEmpty()) return false;
            if (U.getLong(te, O_TE_LCU) != -1L) return false;
            if (U.getObject(te, O_TE_LTC) != null) return false;
            ttGate++;
            return true;
        }
        if (!TT_HANDLES_OK) return false;
        try {
            if (chunk != null) return false;
            java.util.Set<?> seen = (java.util.Set<?>) VH_TE_SEEN.getObj(te);
            if (seen == null || !seen.isEmpty()) return false;
            if (VH_TE_LCU.getLong(te) != -1L) return false;
            if (VH_TE_LTC.getObj(te) != null) return false;
            ttGate++;
            return true;
        } catch (Throwable gateFail) {
            return false; // route vanilla — never break tracking
        }
    }

    // ==================================================================
    // L30 — the tracker-loop floor. The vanilla ChunkMap.newTrackerTick
    // (javap, 26.3) per entity: moonrise$getTrackedEntity (interface),
    // moonrise$getChunkData().nearbyPlayers, moonrise$tick(chunk), then
    // moonrise$hasPlayers()/chunk-status gate + sendChanges. With NO
    // players anywhere the whole per-entity cost collapses: javap
    // moonrise$tick proves the bookkeeping writes are value-invisible
    // while seenBy is empty (clearPlayers early-outs on empty seenBy —
    // bc 16-24; the removal pass runs only when the stamp compare fails
    // and is a no-op over an empty seenBy; the only other readers of
    // lastChunkUpdate/lastTrackedChunk are this gate's own probes, which
    // degrade to vanilla on a stale value), and sendChanges' empty-
    // tracking tail is the L24a-proven updateDataBeforeSync + tickCount++
    // pair. The replacement body below is the byte-exact vanilla loop
    // (the same ReferenceList raw walk, the same iteratingTrackerEntities
    // try/finally protocol, the same hasPlayers/chunk-status gate) with
    // ONE fused fast path: no chunk players AND seenBy empty AND
    // trackedPlayers empty AND interp NO_OP AND !hardColliding AND
    // !syncVelocity -> updateDataBeforeSync + tickCount++, skip the rest.
    // TTL_ON gates ONLY the fast path; the mirror is always the body.
    // ==================================================================

    public static volatile boolean TTL_ON = false;
    public static volatile boolean TTL_HANDLES_OK = false;
    public static final boolean TTL_FLAG_ON =
            !"false".equals(System.getProperty("centities.trackerLoop", "true"));
    public static long ttlCalls = 0, ttlFuse = 0, ttlFuseChecks = 0;

    private static volatile RefField VH_CM_TRACKERS;  // ChunkMap.trackerEntities
    private static volatile RefField VH_CM_ITERATING; // ChunkMap.iteratingTrackerEntities
    private static volatile RefField VH_TE_SERVERENT; // TrackedEntity.serverEntity (public final)
    private static long O_TE_SE = -1;                 // the same field's direct offset
    private static long O_ENT_TRACKED = -1;           // Entity.trackedEntity (private final)

    public static void initTrackerLoopHandles() {
        try {
            Class<?> cm = Class.forName("net.minecraft.server.level.ChunkMap");
            VH_CM_TRACKERS = new RefField(cm, "trackerEntities");
            VH_CM_ITERATING = new RefField(cm, "iteratingTrackerEntities");
            Class<?> te = Class.forName("net.minecraft.server.level.ChunkMap$TrackedEntity");
            VH_TE_SERVERENT = new RefField(te, "serverEntity");
            O_TE_SE = VH_TE_SERVERENT.off;
            O_ENT_TRACKED = offOf(Entity.class, "trackedEntity");
            TTL_HANDLES_OK = VH_CM_TRACKERS.off >= 0 && VH_CM_ITERATING.off >= 0
                    && O_TE_SE >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initTrackerLoopHandles: " + t;
            TTL_HANDLES_OK = false;
        }
    }

    public static void setTrackerLoopOn(boolean on) {
        TTL_ON = on;
    }

    /** The iteratingTrackerEntities write. The field is a BOOLEAN — a raw
     *  Unsafe.putInt would store 4 bytes into a 1-byte slot (the SE_FAST
     *  hazard class), so this goes through the VarHandle (or reflection)
     *  which types the store. Called twice per server tick — cold. */
    private static void setIterating(Object cmObj, boolean v) {
        RefField rf = VH_CM_ITERATING;
        try {
            if (rf.vh != null) {
                rf.vh.set(cmObj, v);
                return;
            }
            rf.f.setBoolean(cmObj, v);
        } catch (IllegalAccessException neverWithSetAccessible) {
            throw new IllegalStateException(neverWithSetAccessible);
        }
    }

    /** The L30 fused fast path. All reads are pure (the L15b REORDER rule —
     *  running the probes BEFORE vanilla's moonrise$tick order is free);
     *  a decline routes the exact vanilla pair. */
    private static boolean ttlFastPath(Entity ent, Object te, Object chunk) {
        ttlFuseChecks++;
        if (!TTL_ON || !TTL_HANDLES_OK || U == null) return false;
        if (O_TE_SEEN <= 0 || O_TE_SE <= 0 || O_SE_TRACKED_PLAYERS <= 0
                || O_SE_INTERP <= 0 || O_SE_TICKCOUNT <= 0) {
            return false;
        }
        try {
            // 1. the chunk side: the VIEW_DISTANCE player list. null chunk
            //    or null/empty list = the clearPlayers-or-nothing case.
            if (chunk != null) {
                ca.spottedleaf.moonrise.common.misc.NearbyPlayers.TrackedChunk tc =
                        (ca.spottedleaf.moonrise.common.misc.NearbyPlayers.TrackedChunk) chunk;
                ca.spottedleaf.moonrise.common.list.ReferenceList<?> players =
                        tc.getPlayers(ca.spottedleaf.moonrise.common.misc.NearbyPlayers.NearbyMapType.VIEW_DISTANCE);
                if (players != null && players.size() != 0) return false;
            }
            // 2. seenBy empty — no removals could fire, updatePlayer loop
            //    provably empty.
            Object seen = U.getObject(te, O_TE_SEEN);
            if (!(seen instanceof java.util.Set<?> ss) || !ss.isEmpty()) return false;
            // 3. the ServerEntity side: the sendChanges fast-exit shape
            //    (the L24a conditions — same equivalence, now from the loop).
            Object se = U.getObject(te, O_TE_SE);
            if (!(se instanceof net.minecraft.server.level.ServerEntity serverEntity)) {
                return false;
            }
            Object trk = U.getObject(serverEntity, O_SE_TRACKED_PLAYERS);
            if (!(trk instanceof java.util.Set<?> ts) || !ts.isEmpty()) return false;
            if (U.getObject(serverEntity, O_SE_INTERP)
                    != net.minecraft.world.entity.InterpolationTracker.NO_OP) return false;
            if (ent.moonrise$isHardColliding() || ent.syncVelocity) return false;
            // ALL conditions passed: this entity's tracker tick emits
            // nothing. Reproduce the two unconditional effects:
            ent.updateDataBeforeSync();                       // sendChanges bc 19-23
            U.putInt(serverEntity, O_SE_TICKCOUNT,
                    U.getInt(serverEntity, O_SE_TICKCOUNT) + 1); // bc 1106-1113
            return true;
        } catch (Throwable fuseFail) {
            return false; // route the exact vanilla pair — never break tracking
        }
    }

    /** Replacement body of ChunkMap.newTrackerTick() — the byte-exact
     *  vanilla loop (javap-anchored) with the L30 fused fast path. */
    public static void newTrackerTick$(Object cmObj) {
        ttlCalls++;
        setIterating(cmObj, true);
        try {
            Object listObj = VH_CM_TRACKERS.getObj(cmObj);
            ca.spottedleaf.moonrise.common.list.ReferenceList<?> list =
                    (ca.spottedleaf.moonrise.common.list.ReferenceList<?>) listObj;
            // vanilla checkcasts the raw data to Entity[] (bc 14) — the
            // same cast here; a CCE is the honest vanilla-shaped failure.
            Entity[] raw = (Entity[]) list.getRawDataUnchecked();
            int size = list.size();
            for (int i = 0; i < size; i++) {
                Entity ent = raw[i];
                // moonrise$getTrackedEntity is a one-field final read — the
                // direct offset when resolved, the interface call otherwise.
                Object te = (O_ENT_TRACKED >= 0 && U != null)
                        ? U.getObject(ent, O_ENT_TRACKED)
                        : (Object) ((ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerEntity) ent)
                                .moonrise$getTrackedEntity();
                if (te == null) continue;                     // vanilla bc 46-51
                Object chunk = ((ca.spottedleaf.moonrise.patches.chunk_system.entity.ChunkSystemEntity) ent)
                        .moonrise$getChunkData().nearbyPlayers;
                // L30: the fused no-tracking fast path (declines to the
                // exact vanilla pair on any doubt).
                if (ttlFastPath(ent, te, chunk)) {
                    ttlFuse++;
                    continue;
                }
                ((ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerTrackedEntity) te)
                        .moonrise$tick((ca.spottedleaf.moonrise.common.misc.NearbyPlayers.TrackedChunk) chunk);
                // vanilla bc 71-94: sendChanges iff hasPlayers() OR the
                // entity's chunk is entity-ticking — NO negation (the first
                // weave shipped with an inverted !hasPlayers(); players
                // tracking an entity in a non-entity-ticking chunk would
                // have missed every update).
                if (((ca.spottedleaf.moonrise.patches.entity_tracker.EntityTrackerTrackedEntity) te)
                        .moonrise$hasPlayers()
                        || ((ca.spottedleaf.moonrise.patches.chunk_system.entity.ChunkSystemEntity) ent)
                                .moonrise$getChunkStatus()
                                .isOrAfter(net.minecraft.server.level.FullChunkStatus.ENTITY_TICKING)) {
                    ((net.minecraft.server.level.ServerEntity) VH_TE_SERVERENT.getObj(te))
                            .sendChanges();
                }
            }
        } finally {
            setIterating(cmObj, false);
        }
    }

    // ==================================================================
    // L33 — the tickEffects empty-map early-out. Vanilla LivingEntity
    // .tickEffects() (javap, 26.3) with an EMPTY activeEffects map still
    // allocates the keySet iterator, writes tickingEffects true/false and
    // polls the deferred queue — per mob per tick (1.2-1.4% of the mob
    // profiles). The guard replicates the empty case EXACTLY: the loop
    // runs zero iterations (nothing observes tickingEffects=true — the
    // MobEffectInstance.tickServer callbacks are the only in-loop readers
    // and they never fire), so the guard skips straight to the finally's
    // semantics: tickingEffects=false + the deferred poll-run drain, run
    // under the same flag value the vanilla finally drains under. A
    // non-empty map, a shape doubt or any handle failure routes the exact
    // vanilla body (the guard is a prefix, never a replacement).
    // ==================================================================

    public static volatile boolean EFF_ON = false;
    public static volatile boolean EFF_HANDLES_OK = false;
    public static final boolean EFF_FLAG_ON =
            !"false".equals(System.getProperty("centities.tickEffects", "true"));
    public static long effCalls = 0, effGate = 0;
    private static volatile RefField VH_LE_EFFECTS;   // LivingEntity.activeEffects
    private static volatile RefField VH_LE_TICKING;   // LivingEntity.tickingEffects
    private static volatile RefField VH_LE_DEFERRED;  // LivingEntity.deferredEffectUpdates

    public static void initEffHandles() {
        try {
            VH_LE_EFFECTS = new RefField(
                    net.minecraft.world.entity.LivingEntity.class, "activeEffects");
            VH_LE_TICKING = new RefField(
                    net.minecraft.world.entity.LivingEntity.class, "tickingEffects");
            VH_LE_DEFERRED = new RefField(
                    net.minecraft.world.entity.LivingEntity.class, "deferredEffectUpdates");
            EFF_HANDLES_OK = VH_LE_EFFECTS.off >= 0 && VH_LE_TICKING.off >= 0
                    && VH_LE_DEFERRED.off >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initEffHandles: " + t;
            EFF_HANDLES_OK = false;
        }
    }

    public static void setTickEffectsGateOn(boolean on) {
        EFF_ON = on;
    }

    private static void setTickingFlag(net.minecraft.world.entity.LivingEntity e, boolean v) {
        RefField rf = VH_LE_TICKING;
        try {
            if (rf.vh != null) {
                rf.vh.set(e, v);
                return;
            }
            rf.f.setBoolean(e, v);
        } catch (IllegalAccessException neverWithSetAccessible) {
            throw new IllegalStateException(neverWithSetAccessible);
        }
    }

    /** tickEffects() guard: TRUE = the empty-map early-out ran (the vanilla
     *  body is skipped); FALSE = route the exact vanilla body. */
    public static boolean tickEffectsNoOp(net.minecraft.world.entity.LivingEntity e) {
        effCalls++;
        if (!EFF_ON || !EFF_HANDLES_OK) return false;
        try {
            Object m = VH_LE_EFFECTS.getObj(e);
            if (!(m instanceof java.util.Map<?, ?> mm) || !mm.isEmpty()) return false;
            // The vanilla finally semantics, byte-for-byte: the flag write
            // (typed — a raw Unsafe putInt would store 4 bytes into the
            // 1-byte boolean slot) then the poll-run drain. An exception
            // from a runnable propagates EXACTLY like vanilla's finally
            // (already-run runnables stay run, the rest stay queued).
            setTickingFlag(e, false);
            Object q = VH_LE_DEFERRED.getObj(e);
            if (!(q instanceof java.util.Queue<?> qq)) return false; // shape doubt
            effGate++;
            Runnable r;
            while ((r = (Runnable) qq.poll()) != null) {
                r.run();
            }
            return true;
        } catch (Throwable effFail) {
            return false; // route vanilla — never break effects
        }
    }

    /** Replacement body of Entity.setTicksFrozen(int). */
    public static void setTicksFrozen$(Entity e, int i) {
        frozenCalls++;
        if (FROZEN_ON && FROZEN_HANDLES_OK) {
            // L25: the direct-read gate — one typed Unsafe walk instead of
            // the getTicksFrozen() boxed chain. LAYERED after the e2e
            // forensics (a77b3ea: the direct walk served zero catches while
            // the flags shim, which kept BOTH layers, caught 9104 through
            // its proven chain layer): the direct read is a pure bonus on
            // top of the proven L18 gate, never its replacement.
            // frozenDirect attributes which layer actually wins.
            try {
                Object cur = entityDataValueIdx(e, IDX_FROZEN);
                if (cur == null) {
                    cur = entityDataValue(e, Entity.DATA_TICKS_FROZEN);
                }
                if (cur instanceof Integer iv && iv.intValue() == i) {
                    frozenDirect++;
                    frozenGate++;
                    return;
                }
            } catch (Throwable directFail) {
                // fall through to the proven get-chain gate
            }
            try {
                if (e.getTicksFrozen() == i) {
                    frozenGate++;
                    return;
                }
            } catch (Throwable gateFail) {
                // never swallow the write — fall through to the vanilla body
            }
            frozenWrites++;
        }
        ((net.minecraft.network.syncher.SynchedEntityData) VH_ENT_DATA.getObj(e))
                .set(Entity.DATA_TICKS_FROZEN, Integer.valueOf(i));
    }

    /** Replacement body of ConversionTracker.setConverting(boolean). */
    @SuppressWarnings("unchecked")
    public static void setConverting$(Object tracker, boolean z) {
        convCalls++;
        if (CONV_ON && CONV_HANDLES_OK) {
            // L25: the direct-read gate. The receiver is ConversionTracker
            // (weave-guaranteed), so O_CT_MOB/O_CT_ID are the exact
            // single-class offsets (doc-32 rule); the mob/entityData hops
            // are typed-checked, and the value read is the same DataItem
            // field vanilla's set() compares — the skip decision is the
            // one vanilla would make.
            try {
                Object m = U.getObject(tracker, O_CT_MOB);
                if (m instanceof net.minecraft.world.entity.Mob mb) {
                    // L29-converting: one-slot accessor identity cache —
                    // every tracker of a mob type holds the same static
                    // accessor, so the repeat calls read the value by the
                    // frozen index; a different accessor re-resolves (and
                    // a non-accessor routes the proven walk).
                    Object acc = U.getObject(tracker, O_CT_ID);
                    Object cur;
                    if (acc == CONV_ACC_CACHE && CONV_ACC_IDX >= 0) {
                        cur = entityDataValueIdx(mb, CONV_ACC_IDX);
                    } else if (acc instanceof
                            net.minecraft.network.syncher.EntityDataAccessor accE) {
                        CONV_ACC_CACHE = acc;
                        CONV_ACC_IDX = accE.id();
                        cur = entityDataValueIdx(mb, CONV_ACC_IDX);
                    } else {
                        cur = null;
                    }
                    if (cur == null) {
                        cur = entityDataValue(mb, acc);
                    }
                    if (cur instanceof Boolean bv && bv.booleanValue() == z) {
                        convDirect++;
                        convGate++;
                        return;
                    }
                }
            } catch (Throwable directFail) {
                // fall through to the proven get-chain gate
            }
            try {
                net.minecraft.world.entity.Mob m2 =
                        (net.minecraft.world.entity.Mob) VH_CONV_MOB.getObj(tracker);
                net.minecraft.network.syncher.EntityDataAccessor<Boolean> id2 =
                        (net.minecraft.network.syncher.EntityDataAccessor<Boolean>)
                                VH_CONV_ID.getObj(tracker);
                Boolean cur2 = m2.getEntityData().get(id2);
                if (cur2.booleanValue() == z) {
                    convGate++;
                    return;
                }
            } catch (Throwable gateFail) {
                // fall through to the vanilla write
            }
            convWrites++;
        }
        net.minecraft.world.entity.Mob m =
                (net.minecraft.world.entity.Mob) VH_CONV_MOB.getObj(tracker);
        net.minecraft.network.syncher.EntityDataAccessor<Boolean> id =
                (net.minecraft.network.syncher.EntityDataAccessor<Boolean>)
                        VH_CONV_ID.getObj(tracker);
        m.getEntityData().set(id, z);
    }

    // ==================================================================
    // L36 — the idle AI-control floors (nav/move/look early-outs).
    //
    // A standing mob's serverAiStep tail calls three control objects every
    // tick. javap (26.3) proves each body collapses to same-value writes
    // for a settled, pathless mob:
    //
    //  - PathNavigation.tick: the whole body is under `if (!isDone())`
    //    (bc 21-25; isDone == path==null || path.isDone()), and the
    //    unconditional prefix is `tick++` (bc 0-7) plus the
    //    hasDelayedRecomputation recompute branch (bc 10-19). With
    //    path==null && !hasDelayedRecomputation the body only ticks its
    //    PRIVATE counter — no observable state.
    //  - MoveControl.tick: the WAIT branch (bc 579-584) is exactly
    //    mob.setZza(0.0F) then return; setZza is a plain putfield. With
    //    operation==WAIT && mob.zza==0.0F the write is a same-value store.
    //  - LookControl.tick: resetXRotOnTick() -> setXRot(0F) (same-value
    //    when xRot==0); the cooldown branch is skipped at
    //    lookAtCooldown<=0; the else branch writes yHeadRot =
    //    rotateTowards(yHeadRot, yBodyRot, 10) — from==to is the EXACT
    //    fixed point (Mth.rotateTowards returns `from + 0`); and
    //    clampHeadRotationToBody() is gated on !navigation.isDone().
    //    No-op on xRot==0 && lookAtCooldown<=0 && yHeadRot==yBodyRot &&
    //    navigation.isDone(). isDone() is called virtually (one
    //    monomorphic frame) so a GroundPathNavigation override stays
    //    authoritative.
    //
    // Every guard is best-effort: a handle failure or shape doubt routes
    // the exact vanilla body. Kill switches:
    // centities.navIdle / centities.moveIdle / centities.lookIdle.
    // Counters: idle= nav/move/look calls, then the three hit counters
    // (appended to the hooks@ line as idle=).
    // ==================================================================

    public static final boolean NAV_FLAG_ON =
            !"false".equals(System.getProperty("centities.navIdle", "true"));
    public static final boolean MOVE_FLAG_ON =
            !"false".equals(System.getProperty("centities.moveIdle", "true"));
    public static final boolean LOOK_FLAG_ON =
            !"false".equals(System.getProperty("centities.lookIdle", "true"));

    private static volatile RefField VH_NAV_DELAYED, VH_NAV_PATH;
    private static volatile RefField VH_MOVE_OP, VH_MOVE_MOB;
    private static volatile RefField VH_LOOK_CD, VH_LOOK_MOB;
    private static volatile RefField VH_MOB_XROT, VH_MOB_YHEAD, VH_MOB_YBODY, VH_MOB_NAV;
    private static RefField VH_MOB_ZZA;
    private static Object MOVE_OP_WAIT_IDLE; // captured enum constant (identity cmp)
    public static volatile boolean NAV_HANDLES_OK, MOVE_HANDLES_OK, LOOK_HANDLES_OK;

    // PLAIN longs (same reasoning as gsFloorTicks — server-thread only).
    public static long navCalls = 0, navIdleHits = 0;
    public static long moveCalls = 0, moveIdleHits = 0;
    public static long lookCalls = 0, lookIdleHits = 0;

    public static void initNavHandles() {
        try {
            VH_NAV_DELAYED = new RefField(
                    net.minecraft.world.entity.ai.navigation.PathNavigation.class,
                    "hasDelayedRecomputation");
            VH_NAV_PATH = new RefField(
                    net.minecraft.world.entity.ai.navigation.PathNavigation.class,
                    "path");
            NAV_HANDLES_OK = VH_NAV_DELAYED.off >= 0 && VH_NAV_PATH.off >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initNavHandles: " + t;
            NAV_HANDLES_OK = false;
        }
    }

    public static void initMoveHandles() {
        try {
            VH_MOVE_OP = new RefField(
                    net.minecraft.world.entity.ai.control.MoveControl.class,
                    "operation");
            VH_MOVE_MOB = new RefField(
                    net.minecraft.world.entity.ai.control.MoveControl.class,
                    "mob");
            // Operation is PROTECTED in MoveControl (javap) — capture via
            // reflection, same as the L22 init's Enum.valueOf capture.
            MOVE_OP_WAIT_IDLE = java.lang.Enum.valueOf(
                    (Class<? extends java.lang.Enum>) Class.forName(
                            "net.minecraft.world.entity.ai.control.MoveControl$Operation"),
                    "WAIT");
            MOVE_HANDLES_OK = VH_MOVE_OP.off >= 0 && VH_MOVE_MOB.off >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initMoveHandles: " + t;
            MOVE_HANDLES_OK = false;
        }
    }

    public static void initLookHandles() {
        try {
            VH_LOOK_CD = new RefField(
                    net.minecraft.world.entity.ai.control.LookControl.class,
                    "lookAtCooldown");
            VH_LOOK_MOB = new RefField(
                    net.minecraft.world.entity.ai.control.LookControl.class,
                    "mob");
            VH_MOB_XROT = new RefField(net.minecraft.world.entity.Entity.class, "xRot");
            VH_MOB_YHEAD = new RefField(net.minecraft.world.entity.Mob.class, "yHeadRot");
            VH_MOB_YBODY = new RefField(net.minecraft.world.entity.Mob.class, "yBodyRot");
            VH_MOB_NAV = new RefField(net.minecraft.world.entity.Mob.class, "navigation");
            VH_MOB_ZZA = new RefField(net.minecraft.world.entity.LivingEntity.class, "zza");
            LOOK_HANDLES_OK = VH_LOOK_CD.off >= 0 && VH_LOOK_MOB.off >= 0
                    && VH_MOB_XROT.off >= 0 && VH_MOB_YHEAD.off >= 0
                    && VH_MOB_YBODY.off >= 0 && VH_MOB_NAV.off >= 0
                    && VH_MOB_ZZA.off >= 0;
        } catch (Throwable t) {
            lastError = System.nanoTime();
            lastErrorText = "initLookHandles: " + t;
            LOOK_HANDLES_OK = false;
        }
    }

    /** PathNavigation.tick() guard: TRUE = the body is a provable no-op. */
    public static boolean navIdle$(net.minecraft.world.entity.ai.navigation.PathNavigation nav) {
        navCalls++;
        if (!NAV_FLAG_ON || !NAV_HANDLES_OK) return false;
        try {
            if (VH_NAV_DELAYED.getBool(nav)) return false;      // recompute pending
            if (VH_NAV_PATH.getObj(nav) != null) return false;  // active path
            navIdleHits++;
            return true;
        } catch (Throwable t) {
            return false; // route vanilla — never break navigation
        }
    }

    /** MoveControl.tick() guard: TRUE = the body is a provable no-op. */
    public static boolean moveIdle$(net.minecraft.world.entity.ai.control.MoveControl mc) {
        moveCalls++;
        if (!MOVE_FLAG_ON || !MOVE_HANDLES_OK) return false;
        try {
            if (VH_MOVE_OP.getObj(mc) != MOVE_OP_WAIT_IDLE) return false;
            Object mob = VH_MOVE_MOB.getObj(mc);
            if (!(mob instanceof net.minecraft.world.entity.Mob)) return false;
            if (VH_MOB_ZZA.getFloat(mob) != 0.0f) return false;
            moveIdleHits++;
            return true;
        } catch (Throwable t) {
            return false; // route vanilla — never break movement
        }
    }

    /** LookControl.tick() guard: TRUE = the body is a provable no-op. */
    public static boolean lookIdle$(net.minecraft.world.entity.ai.control.LookControl lc) {
        lookCalls++;
        if (!LOOK_FLAG_ON || !LOOK_HANDLES_OK) return false;
        try {
            Object mob = VH_LOOK_MOB.getObj(lc);
            if (!(mob instanceof net.minecraft.world.entity.Mob)) return false;
            if (VH_LOOK_CD.getInt(lc) > 0) return false;          // look target active
            if (VH_MOB_XROT.getFloat(mob) != 0.0f) return false;  // pitch reset pending
            if (VH_MOB_YHEAD.getFloat(mob) != VH_MOB_YBODY.getFloat(mob)) return false;
            Object nav = VH_MOB_NAV.getObj(mob);
            if (!(nav instanceof net.minecraft.world.entity.ai.navigation.PathNavigation pn)
                    || !pn.isDone()) {
                return false; // head clamp pending while a path is live
            }
            lookIdleHits++;
            return true;
        } catch (Throwable t) {
            return false; // route vanilla — never break look control
        }
    }
}
