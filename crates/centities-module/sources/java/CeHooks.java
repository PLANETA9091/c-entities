package dev.planeta9091.centities;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.phys.AABB;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

/**
 * Bridge class defined into the kernel loader by the c-entities module.
 * Static methods here are (a) a replace_body target (getEntities),
 * (b) prefix probes (add/remove/pos — the original body always runs after),
 * or (c) natives (Rust).
 *
 * FALLBACK CONTRACT: when ACTIVE is false every woven path behaves exactly
 * like vanilla (the e2e differential test proves it before arming).
 */
public final class CeHooks {
    private CeHooks() {}

    /** Kill switch. Flipped to false by any anomaly; never re-armed in-process. */
    public static volatile boolean ACTIVE = false;

    // Plain (non-volatile) counters: diagnostic only, and volatile writes on
    // every probe/query would put a membar on the hottest paths across all
    // ticking threads. Races on these longs are acceptable.
    public static long queriesFast = 0, queriesMirror = 0;
    public static long probesAdd = 0, probesRemove = 0, probesPos = 0, posSkips = 0;
    public static long posDedupHits = 0, posNative = 0;

    // ------------------------------------------------------------------
    // Woven (replace_body): Level.getEntities(Entity, AABB, Predicate)
    // patched-method slots: 0=this, 1=source, 2=box, 3=predicate
    // ------------------------------------------------------------------
    public static List<Entity> getEntities(
            net.minecraft.world.level.Level level,
            Entity source, AABB box, Predicate<? super Entity> predicate) {
        if (ACTIVE) {
            try {
                List<Entity> out = fastQuery(level, box, predicate);
                if (out != null) {
                    queriesFast++;
                    return out;
                }
            } catch (Throwable t) {
                ACTIVE = false; // never break the server
            }
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
    // Woven (replace_body): Level.getEntitiesOfClass(Class, AABB, Predicate)
    // patched-method slots: 0=this, 1=clazz, 2=box, 3=predicate
    // ------------------------------------------------------------------
    public static <T extends Entity> java.util.List<T> getEntitiesOfClass(
            net.minecraft.world.level.Level level, Class<T> clazz, AABB box,
            Predicate<? super T> predicate) {
        if (ACTIVE) {
            try {
                List<T> out = fastQueryTyped(level, clazz, box, predicate);
                if (out != null) {
                    queriesFast++;
                    return out;
                }
            } catch (Throwable t) {
                ACTIVE = false; // never break the server
            }
        }
        queriesMirror++;
        // Vanilla mirror — identical to the replaced body of
        // Level.getEntitiesOfClass(Class, AABB, Predicate), Paper 1.21.8
        // (decompiled from the shipped jar; keep in sync).
        List<T> ret = new ArrayList<>();
        ((net.minecraft.server.level.ServerLevel) level)
                .moonrise$getEntityLookup().getEntities(clazz, null, box, ret, predicate);
        return ret;
    }

    // ------------------------------------------------------------------
    // Prefix probes: inserted before the original body, stack-neutral.
    // ------------------------------------------------------------------

    /** Prefix of EntityLookup.addEntity(Entity,boolean,boolean): slot 1 = entity. */
    public static void probeAdd(Entity e) {
        try {
            if (ACTIVE && e != null && e.level() != null && !e.level().isClientSide) {
                probesAdd++;
                AABB b = e.getBoundingBox();
                nativeAdd(e.getId(), worldId(e.level()), b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
            }
        } catch (Throwable t) {
            ACTIVE = false;
        }
    }

    /** Prefix of EntityLookup.removeEntity(Entity): slot 1 = entity. */
    public static void probeRemove(Entity e) {
        try {
            if (ACTIVE) {
                probesRemove++;
                nativeRemove(e.getId());
            }
        } catch (Throwable t) {
            ACTIVE = false;
        }
    }

    /** Prefix of Entity.setBoundingBox(AABB): the NEW box arrives as an arg. */
    public static void probeBox(Entity e, AABB newBox) {
        try {
            if (ACTIVE && e != null && newBox != null && e.level() != null && !e.level().isClientSide) {
                nativeBox(e.getId(), worldId(e.level()),
                        newBox.minX, newBox.minY, newBox.minZ, newBox.maxX, newBox.maxY, newBox.maxZ);
            }
        } catch (Throwable t) {
            ACTIVE = false;
        }
    }

    /**
     * Prefix of Entity.setPos(double,double,double):
     * args (this, x, y, z). Runs on EVERY setPos — the native side
     * early-outs on unchanged coordinates in ~tens of ns.
     *
     * JAVA-SIDE DEDUP: most entities (grounded items, idle mobs) call
     * setPos with unchanged coordinates every tick; a JNI transition per
     * call (~50–80 ns) costs more than the whole index save. An open-
     * addressed long-array cache (id → exact x/y/z bits) filters no-op
     * probes in ~10 ns of pure Java. Plain (non-volatile) arrays are safe:
     * an entity ticks on exactly one region thread, so every read sees
     * that thread's own writes.
     */
    public static void probePos(Entity e, double x, double y, double z) {
        try {
            if (!ACTIVE || e == null) return;
            net.minecraft.world.level.Level l = e.level();
            if (l == null || l.isClientSide) return;
            probesPos++;
            int h = posSlot(e.getId());
            long xb = Double.doubleToLongBits(x), yb = Double.doubleToLongBits(y), zb = Double.doubleToLongBits(z);
            if (POS_ID[h] == e.getId() && POS_X[h] == xb && POS_Y[h] == yb && POS_Z[h] == zb) {
                posDedupHits++;
                return; // exact same position as last seen — index already correct
            }
            posNative++;
            POS_ID[h] = e.getId();
            POS_X[h] = xb;
            POS_Y[h] = yb;
            POS_Z[h] = zb;
            AABB b = e.getBoundingBox(); // OLD box: constant dims assumed, corrected by probeBox
            nativePos(e.getId(), worldId(l), x, y, z, b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
        } catch (Throwable t) {
            ACTIVE = false;
        }
    }

    private static final int POS_MASK = 0x3FFFF;
    private static final long[] POS_ID = new long[POS_MASK + 1];
    private static final long[] POS_X = new long[POS_MASK + 1];
    private static final long[] POS_Y = new long[POS_MASK + 1];
    private static final long[] POS_Z = new long[POS_MASK + 1];

    private static int posSlot(long id) {
        return (int) ((id * 0x9E3779B97F4A7C15L) >>> 46) & POS_MASK;
    }

    /** Kill-switch arm/disarm from the native side. */
    public static void setActive(boolean on) {
        ACTIVE = on;
    }

    /** 1 once the server finished booting (all worlds present, ticking). */
    public static int serverReady() {
        net.minecraft.server.MinecraftServer s = net.minecraft.server.MinecraftServer.getServer();
        return s != null && s.isReady() ? 1 : 0;
    }

    /**
     * One-shot index fill from the live world state. Called from the native
     * arming thread AFTER the server is fully booted, BEFORE ACTIVE flips:
     * every entity in every level lands in the index, then the setPos/box
     * probes (already woven) keep it fresh, and only afterwards does the
     * Level query swap in. No stale window, no missed entities.
     */
    public static void bootstrap() {
        for (net.minecraft.server.level.ServerLevel sl : net.minecraft.server.MinecraftServer.getServer().getAllLevels()) {
            int wid = worldId(sl);
            for (Entity e : sl.moonrise$getEntityLookup().getAll()) {
                try {
                    if (e == null) continue;
                    net.minecraft.world.phys.AABB b = e.getBoundingBox();
                    nativeAdd(e.getId(), wid, b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
                } catch (Throwable t) {
                    ACTIVE = false;
                    return;
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Fast query: Rust index -> id candidates -> Java resolve/recheck.
    // Returns null => caller takes the mirror path.
    //
    // COST HEURISTIC: the native reply carries [0]=candidate count and
    // [1]=entries walked. Resolving a candidate in Java (id map + recheck)
    // costs ~10x a vanilla inline visit, so when candidates are numerous
    // (wide sensor boxes in dense cells) the vanilla slice walk wins and we
    // take the mirror. Thresholds: hard cap on candidates; density rule
    // (candidates > visited/2) covers mid-size boxes in packed cells.
    // ------------------------------------------------------------------
    private static final int RESOLVE_CAP = 512;
    public static long queriesFallback = 0;

    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, AABB box, Predicate<? super Entity> predicate) {
        if (!(level instanceof net.minecraft.server.level.ServerLevel sl)) return null;
        int wid = worldId(sl);
        if (wid < 0) return null;
        long[] reply = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ);
        if (reply == null) return null;
        int n = (int) reply[0];
        long visited = reply[1];
        if (n < 0 || n > RESOLVE_CAP || (n > 64 && n * 2 > visited)) {
            queriesFallback++;
            return null; // vanilla slice walk is cheaper for this query shape
        }

        ArrayList<Entity> out = new ArrayList<>(Math.min(n, 64));
        int resolved = 0;
        for (int i = 2; i < 2 + n; i++) {
            Entity e = sl.getEntity((int) reply[i]); // LevelEntityGetter.byId — id-keyed
            if (e == null) continue; // stale candidate: filtered by the recheck below
            resolved++;
            if ((predicate == null || predicate.test(e))
                    && box.intersects(e.getBoundingBox())) {
                out.add(e);
            }
        }
        if (resolved == 0 && n > 0) return null; // desync guard
        return out;
    }

    /** Typed variant of {@link #fastQuery} for getEntitiesOfClass: adds the
     * class check the vanilla EntityLookup.getEntities(Class,...) applies. */
    private static <T extends Entity> List<T> fastQueryTyped(
            net.minecraft.world.level.Level level, Class<T> clazz, AABB box,
            Predicate<? super T> predicate) {
        if (!(level instanceof net.minecraft.server.level.ServerLevel sl)) return null;
        int wid = worldId(sl);
        if (wid < 0) return null;
        long[] reply = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ);
        if (reply == null) return null;
        int n = (int) reply[0];
        long visited = reply[1];
        if (n < 0 || n > RESOLVE_CAP || (n > 64 && n * 2 > visited)) {
            queriesFallback++;
            return null;
        }

        ArrayList<T> out = new ArrayList<>(Math.min(n, 64));
        int resolved = 0;
        for (int i = 2; i < 2 + n; i++) {
            Entity e = sl.getEntity((int) reply[i]);
            if (e == null) continue;
            resolved++;
            if (clazz.isInstance(e)
                    && (predicate == null || predicate.test(clazz.cast(e)))
                    && box.intersects(e.getBoundingBox())) {
                out.add(clazz.cast(e));
            }
        }
        if (resolved == 0 && n > 0) return null; // desync guard
        return out;
    }

    // ------------------------------------------------------------------
    // World registry (main-thread discipline)
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

    private static int worldIdOf(Entity e) {
        return worldId(e.level());
    }

    // ------------------------------------------------------------------
    // Natives (Rust)
    // ------------------------------------------------------------------
    public static native long[] queryNative(
            int world, double x0, double y0, double z0, double x1, double y1, double z1);
    public static native void nativeAdd(long id, int world, double x0, double y0, double z0,
            double x1, double y1, double z1);
    public static native void nativeBox(long id, int world, double x0, double y0, double z0,
            double x1, double y1, double z1);
    public static native void nativeRemove(long id);
    public static native void nativePos(long id, int world, double x, double y, double z,
            double x0, double y0, double z0, double x1, double y1, double z1);
}
