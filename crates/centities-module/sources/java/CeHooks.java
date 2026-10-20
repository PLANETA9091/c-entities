package dev.planeta9091.centities;

import net.minecraft.world.entity.Entity;
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
                + " indexing=" + INDEXING
                + " active=" + ACTIVE + " lastErr=" + lastErrorText;
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
                        if (me > 0 && dt > 2 * me) {
                            if (bk < 4) bk++;
                            // Soft unstick: jump half-way toward the sample,
                            // NOT to the sample. A single expensive re-probe
                            // must not wipe the EMA's memory of a bucket where
                            // fast genuinely wins most of the time (4a.1 did
                            // that and pushed spread-100k into a permanent
                            // mirror + suspend/resync storm).
                            if (dt > 4 * FAST_EMA[gi]) FAST_EMA[gi] = (FAST_EMA[gi] + dt) / 2;
                        } else if (bk > 0 && me > 0 && dt < me) {
                            bk--; // cheap again — restore probing granularity
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

    /** Prefix of EntityLookup.addEntity(Entity,boolean,boolean): slot 1 = entity. */
    public static void probeAdd(Entity e) {
        try {
            if (ACTIVE && INDEXING && e != null && e.level() != null && !e.level().isClientSide) {
                probesAdd++;
                ARENA[e.getId() & AMASK] = e;
                batchUpsert(e.getId(), worldId(e.level()), e.getBoundingBox());
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
                batchUpsert(e.getId(), worldId(e.level()), newBox);
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

    private static void batchUpsert(int id, int wid, AABB b) {
        if (!SERVER_THREAD.get()) {
            nativeBoxDirect(id, wid,
                    b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
            return;
        }
        B_IDS[bN] = ((wid & 0xFFFFL) << 32) | (id & 0xFFFF_FFFFL);
        int o = bN * 6;
        B_BOX[o] = b.minX; B_BOX[o + 1] = b.minY; B_BOX[o + 2] = b.minZ;
        B_BOX[o + 3] = b.maxX; B_BOX[o + 4] = b.maxY; B_BOX[o + 5] = b.maxZ;
        bN++;
        if (bN >= BATCH_CAP - BATCH_CAP / 4) flush();
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
    // PARITY GATE (iteration 4). Owner's hard requirement: the module must
    // never be slower than vanilla. Per (world, volume-bucket) we keep an
    // EMA of measured fast-path and mirror-path nanos; every query is routed
    // to the measured-cheaper path. Fast wins only in dense buckets (the
    // exact Rust index skips the section scan's wasted intersects); small
    // queries stay vanilla. When fast queries all but disappear across the
    // whole gate window, indexing suspends entirely (probes = 1 volatile
    // read) and re-arms through a full resync when an expensive query shows
    // up again. Worst case is therefore vanilla parity minus gate arithmetic.
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
    /** Re-probe backoff per cell (0..4): when a re-probe lands on a much
     *  more expensive fast sample than mirror, the 1/256 probe rate itself
     *  becomes a tax (mixed-150k: 66 probes/tick × 307us = 20 ms/tick). The
     *  step doubles up to 1/4096 until fast gets cheap again. */
    private static final int[] FAST_BACKOFF = new int[GWORLDS * GBUCKETS];
    private static int gateDecisions = 0, gateFastChosen = 0;
    private static final int GATE_WINDOW = 16_384;
    private static final int GATE_MIN_FAST = 32;   // below: fast is useless
    private static final long RESUME_NANOS = 30_000; // an expensive query re-arms
    /** Last sampled mirror price (0 = none). A suspend must not fire while
     *  mirror queries are this expensive — the world is dense enough that a
     *  re-synced index may win; suspending then causes a suspend<->resync
     *  cycle (spread-100k 4a.1: 65.8k batch entries/tick = resync every
     *  tick). */
    private static volatile long lastMirrorNanos = 0;
    /** Gate windows that must pass (with a live index) before suspend is
     *  allowed again — set by resumeIndexing. Without it the gate drops the
     *  index and resumes it in the same tick: suspend<->resync storm. */
    private static volatile int suspendCooldown = 0;

    /** INDEXING=false means probes do nothing (suspend). SUSPENDED means the
     *  gate also refuses fast queries until a full resync re-arms the index. */
    private static volatile boolean INDEXING = true;
    private static volatile boolean SUSPENDED = false;

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
        if (SUSPENDED) {
            gateDecisions++;
            return -1; // resume trigger lives in mirrorMaybeTimed
        }
        return wid * GBUCKETS + bucketOf(box);
    }

    /** The per-cell routing decision. FAST_N < 16 bootstraps with measured
     *  probes; afterwards a 1/256 re-probe keeps EMAs from going stale and
     *  the comparison runs with hysteresis (fast must be < 80% of mirror).
     *  Also feeds the suspend window. */
    private static boolean wantFast(int gi) {
        boolean fast;
        if (FAST_N[gi] < 16) {
            fast = true; // bootstrap: a few measured probes per bucket
        } else {
            int step = 256 << Math.min(FAST_BACKOFF[gi], 4); // 256..4096
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
        } else {
            maybeSuspend();
        }
        return fast;
    }

    /** Mirror with sampled timing: the EMA needs mirror prices, and while
     *  suspended a single expensive sample is the resume trigger. gi >= 0
     *  means the query has a gate cell (per-bucket sampling applies);
     *  gi < 0 means vanilla outright (sample only for the resume scan). */
    private static List<Entity> mirrorMaybeTimed(
            net.minecraft.world.level.Level level,
            Entity source, AABB box, Predicate<? super Entity> predicate, int gi) {
        boolean sample;
        if (gi < 0) {
            sample = SUSPENDED && (mirrorProbes++ & 63) == 0; // resume scan while asleep
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
        lastMirrorNanos = dt;
        if (gi >= 0) {
            ema(MIRR_EMA, gi, dt);
            if (MIRR_N[gi] < 1_000_000) MIRR_N[gi]++;
        }
        if (SUSPENDED && dt >= RESUME_NANOS) {
            resumeIndexing();
        }
        return ret;
    }

    private static void maybeSuspend() {
        if (gateDecisions < GATE_WINDOW) return;
        int chosen = gateFastChosen;
        gateDecisions = 0;
        gateFastChosen = 0;
        if (SUSPENDED) return;
        // Dense world in mirror mode: dropping the index now guarantees a
        // suspend<->resync cycle (resume re-arms on the very next expensive
        // sample). Keep indexing and let the gate keep measuring.
        if (lastMirrorNanos >= RESUME_NANOS) return;
        // Post-resume cooldown: judge the freshly re-synced index over 4
        // full windows before suspend may fire again.
        if (suspendCooldown > 0) {
            suspendCooldown--;
            return;
        }
        if (chosen < GATE_MIN_FAST) {
            SUSPENDED = true;
            INDEXING = false;
            bN = 0; // pending deltas are irrelevant: the index is dropped and
            nativeReset(); // will be fully rebuilt by resync on resume
            java.util.Arrays.fill(ARENA, null); // release entity refs for GC
            suspends++;
        }
    }

    /** Full re-sync of the Rust index + ARENA from the live lookups. Runs on
     *  the calling (server) thread; vanilla's getAllEntities iterates safely.
     *  Off-thread direct probes racing the resync are harmless: worst case a
     *  candidate is briefly stale and resolve falls back or skips it. */
    private static void resumeIndexing() {
        nativeReset();
        for (net.minecraft.world.level.Level w : WORLDS.keySet()) {
            if (!(w instanceof net.minecraft.server.level.ServerLevel sl)) continue;
            int wid = worldId(sl);
            int n = 0;
            for (Entity e : sl.getAllEntities()) {
                if (e == null || e.isRemoved()) continue;
                AABB b = e.getBoundingBox();
                ARENA[e.getId() & AMASK] = e;
                batchUpsert(e.getId(), wid, b);
                n++;
            }
            resyncEnts += n;
        }
        flush();
        INDEXING = true;
        SUSPENDED = false;
        resumes++;
        suspendCooldown = 4;
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
        int wid = worldId(sl);
        if (wid < 0) {
            mBadWorld++;
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
