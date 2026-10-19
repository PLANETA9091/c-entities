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
                long t0 = System.nanoTime();
                List<Entity> out = fastQuery(level, box, predicate);
                fastNanos += System.nanoTime() - t0;
                if (out != null) {
                    queriesFast++;
                    return out;
                }
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
        // Vanilla mirror — identical to the replaced body of
        // Level.getEntities(Entity, AABB, Predicate), Paper 1.21.8
        // (decompiled from the shipped jar; keep in sync).
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
            if (ACTIVE && e != null && e.level() != null && !e.level().isClientSide) {
                probesAdd++;
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
            if (ACTIVE) {
                probesRemove++;
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
            if (ACTIVE && e != null && newBox != null && e.level() != null && !e.level().isClientSide) {
                probesBox++;
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
    // Fast query: flush pending probes, Rust exact query -> resolve ids.
    // Returns null => caller takes the mirror path.
    // ------------------------------------------------------------------
    private static final ThreadLocal<long[]> QBUF =
            ThreadLocal.withInitial(() -> new long[1024]);

    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, AABB box, Predicate<? super Entity> predicate) {
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
        long[] ids = QBUF.get();
        int bn = bN;
        long t1 = System.nanoTime();
        int n = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                ids, bn, B_IDS, B_BOX);
        queryNanos += System.nanoTime() - t1;
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
        candidatesSeen += n;
        long t2 = System.nanoTime();
        for (int k = 0; k < n; k++) {
            Entity e = sl.getEntity((int) ids[k]); // LevelEntityGetter.byId — id-keyed
            if (e == null) continue; // stale candidate (removed before its probe ran)
            resolved++;
            // NOTE: no per-candidate box.intersects — the Rust index is exact
            // (property-tested vs brute force in centities-core); the vanilla
            // predicate still applies, as it must.
            if (predicate.test(e)) {
                out.add(e);
            }
        }
        resolveNanos += System.nanoTime() - t2;
        candidatesResolved += resolved;
        if (resolved == 0 && n > 0) {
            mDesync++;
            return null; // desync guard
        }
        return out;
    }

    // ------------------------------------------------------------------
    // World registry
    // ------------------------------------------------------------------
    private static final java.util.IdentityHashMap<net.minecraft.world.level.Level, Integer> WORLDS =
            new java.util.IdentityHashMap<>();
    private static int nextWorld = 0;

    public static int worldId(net.minecraft.world.level.Level level) {
        Integer w = WORLDS.get(level);
        if (w != null) return w;
        w = nextWorld++;
        WORLDS.put(level, w);
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
}
