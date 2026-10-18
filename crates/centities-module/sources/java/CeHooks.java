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

    public static volatile long queriesFast = 0, queriesMirror = 0;
    public static volatile long probesAdd = 0, probesRemove = 0, probesPos = 0, posSkips = 0;

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
     */
    public static void probePos(Entity e, double x, double y, double z) {
        try {
            if (!ACTIVE || e == null) return;
            net.minecraft.world.level.Level l = e.level();
            if (l == null || l.isClientSide) return;
            probesPos++;
            AABB b = e.getBoundingBox(); // OLD box: constant dims assumed, corrected by probeBox
            nativePos(e.getId(), worldId(l), x, y, z, b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ);
        } catch (Throwable t) {
            ACTIVE = false;
        }
    }

    /** Kill-switch arm/disarm from the native side. */
    public static void setActive(boolean on) {
        ACTIVE = on;
    }

    // ------------------------------------------------------------------
    // Fast query: Rust index -> id candidates -> Java resolve/recheck.
    // Returns null => caller takes the mirror path.
    // ------------------------------------------------------------------
    private static List<Entity> fastQuery(
            net.minecraft.world.level.Level level, AABB box, Predicate<? super Entity> predicate) {
        if (!(level instanceof net.minecraft.server.level.ServerLevel sl)) return null;
        int wid = worldId(sl);
        if (wid < 0) return null;
        long[] ids = queryNative(wid, box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ);
        if (ids == null) return null;

        var lookup = sl.moonrise$getEntityLookup();
        ArrayList<Entity> out = new ArrayList<>(Math.min(ids.length, 64));
        int resolved = 0;
        for (long id : ids) {
            Entity e = sl.getEntity((int) id); // LevelEntityGetter.byId — id-keyed
            if (e == null) continue; // stale candidate: filtered by the recheck below
            resolved++;
            if (box.intersects(e.getBoundingBox()) && predicate.test(e)) {
                out.add(e);
            }
        }
        if (resolved == 0 && ids.length > 0) return null; // desync guard
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
