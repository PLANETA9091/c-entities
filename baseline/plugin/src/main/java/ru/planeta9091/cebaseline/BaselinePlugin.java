package ru.planeta9091.cebaseline;

import com.destroystokyo.paper.event.server.ServerTickEndEvent;
import org.bukkit.Bukkit;
import org.bukkit.GameRule;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.block.Chest;
import org.bukkit.block.Furnace;
import org.bukkit.block.Hopper;
import org.bukkit.enchantments.Enchantment;
import org.bukkit.entity.ArmorStand;
import org.bukkit.entity.Entity;
import org.bukkit.entity.EntityType;
import org.bukkit.entity.Item;
import org.bukkit.entity.Player;
import org.bukkit.entity.Zombie;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.player.PlayerJoinEvent;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;

import java.io.File;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.security.MessageDigest;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.Deque;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Random;
import java.util.Set;

/**
 * Baseline harness: fills the world with a configurable entity/tile load
 * around a fake player and records per-tick durations. Pure measurement,
 * no optimization. Results -> baseline.json (server dir).
 *
 * Env knobs (all optional):
 *  RUN_NAME, SCALE (entities), FILL_TICKS, WARMUP_TICKS, MEASURE_TICKS,
 *  AREA (entity spread, blocks), INNER_RADIUS (active mob zone),
 *  PCT_ITEMS, PCT_STANDS (rest = zombies), PCT_MERGABLE (share of items
 *  that merge), TILE_HOPPERS, TILE_FURNACES, TILE_AREA.
 *
 *  PARITY=1 switches on the behavioral-equivalence harness: the fill window
 *  starts at a FIXED tick (wallclock-independent, no bot), Paper's shared
 *  entity RNG is re-seeded to a fixed seed at deterministic logical points
 *  (every fill tick + once at fill-done), and full entity-state snapshots
 *  (bit-exact hex fields) are written at fill-done offsets +50/+300 and at
 *  the end. Two servers (module off/on) then MUST produce identical
 *  snapshots — any extra/missing RNG draw, different goal decision or
 *  movement drift in the optimized paths diverges the state and fails the
 *  parity gate.
 */
public class BaselinePlugin extends JavaPlugin implements Listener {

    private final Random rnd = new Random(42L);

    private String runName = "run";
    private int scale = 0;
    private int fillTicks = 400;
    private int warmupTicks = 100;
    private int measureTicks = 600;
    private int area = 150;
    private int innerRadius = 28;
    private double pctItems = 1.0, pctStands = 0.0;
    private double pctMergable = 0.5;
    private int tileHoppers = 0, tileFurnaces = 0, tileArea = 360;

    private boolean started = false;
    private String phase = "WAIT_JOIN";
    private long tickNo = 0;
    private long measureEndTick = -1;
    private int spawnedItems = 0, spawnedStands = 0, spawnedZombies = 0;
    private int placedHoppers = 0, placedFurnaces = 0;

    // --- behavioral parity harness ---
    private boolean parity = false;
    private long paritySeed = 4242L;
    private int parityFillStartTick = 100; // fixed-tick fill window start
    private long fillDoneTick = -1;
    private Object sharedRandom;           // net.minecraft...Entity.SHARED_RANDOM
    private boolean sharedRandomTried;
    private static final int[] PARITY_SNAP_OFFSETS = {50, 300};
    private final Set<Long> snapDone = new HashSet<>();

    private Location anchor = null;
    private int groundY = -60;
    private double entityCarry = 0.0, tileCarry = 0.0;

    private final List<double[]> samples = new ArrayList<>();
    private final List<double[]> tickDurations = new ArrayList<>();

    // tile fill queue: type,x,y,z
    private final Deque<int[]> placeQueue = new ArrayDeque<>();

    @Override
    public void onEnable() {
        runName = env("RUN_NAME", "run");
        scale = intEnv("SCALE", 0);
        fillTicks = Math.max(1, intEnv("FILL_TICKS", 400));
        warmupTicks = intEnv("WARMUP_TICKS", 100);
        measureTicks = intEnv("MEASURE_TICKS", 600);
        area = intEnv("AREA", 150);
        innerRadius = intEnv("INNER_RADIUS", 28);
        pctItems = doubleEnv("PCT_ITEMS", 1.0);
        pctStands = doubleEnv("PCT_STANDS", 0.0);
        pctMergable = doubleEnv("PCT_MERGABLE", 0.5);
        tileHoppers = intEnv("TILE_HOPPERS", 0);
        tileFurnaces = intEnv("TILE_FURNACES", 0);
        tileArea = intEnv("TILE_AREA", 260);
        parity = "1".equals(env("PARITY", "0"));
        paritySeed = longEnv("PARITY_SEED", 4242L);
        traceId = (int) longEnv("PARITY_TRACE_ID", -1L);

        // Neutralize non-entity noise.
        for (World w : Bukkit.getWorlds()) {
            w.setGameRule(GameRule.DO_MOB_SPAWNING, false);
            w.setGameRule(GameRule.DO_DAYLIGHT_CYCLE, false);
            w.setGameRule(GameRule.DO_FIRE_TICK, false);
            w.setGameRule(GameRule.MOB_GRIEFING, false);
            w.setGameRule(GameRule.RANDOM_TICK_SPEED, 0);
            w.setStorm(false);
            w.setThundering(false);
            try {
                w.setTime(6000L);
            } catch (IllegalArgumentException ex) {
                // 26.3: "Cannot set time in world without world clock" — this
                // world has no clock object; DO_DAYLIGHT_CYCLE=false (above)
                // already pins the time, so determinism is preserved.
            }
        }

        getServer().getPluginManager().registerEvents(this, this);

        // Always-on hook diagnostics: every 100 ticks, any phase.
        Bukkit.getScheduler().runTaskTimer(this,
                () -> { String h = hookStats(); if (h != null) getLogger().info("hooks@T" + tickNo + ": " + h); },
                60L, 100L);
        getLogger().info(String.format(Locale.ROOT,
                "ce-baseline config: run=%s scale=%d items=%.2f stands=%.2f tiles(h=%d,f=%d) measure=%d",
                runName, scale, pctItems, pctStands, tileHoppers, tileFurnaces, measureTicks));

        // Failsafe: if the fake player never joins, start anyway (marked no-player).
        // PARITY mode replaces this with its own fixed-tick start (a wallclock
        // 45s failsafe would begin the fill at a DIFFERENT tick in each leg).
        if (!parity) {
            Bukkit.getScheduler().runTaskLater(this, () -> {
                if (!started) {
                    getLogger().warning("no player joined within 45s — starting without player (mobs will be inactive)");
                    begin(null);
                }
            }, 20L * 45L);
        }

        if (parity) {
            // Parity mode: NO bot (player-driven motion is wallclock-random).
            // The fill window starts at a fixed TICK so both legs of the A/B
            // begin spawning on the same tick number regardless of boot speed.
            //
            // SEEDING (javap, 26.3): Entity.SHARED_RANDOM is an
            // Entity$RandomRandomSource whose setSeed LOCKS on first use —
            // later calls are logged and IGNORED ("Ignoring setSeed on
            // Entity.SHARED_RANDOM"). One seed, applied here before any
            // entity exists, is the ONLY lever — and it is sufficient: the
            // fixed-tick fill window pins the spawn schedule to identical
            // tick numbers in both legs, and from there the single global
            // stream evolves identically tick-by-tick. Any behavioral
            // difference (extra draw, different decision) diverges it.
            boolean seeded = seedSharedRandom("onEnable");
            boolean lvlSeeded = seedLevelRandom("onEnable");
            getLogger().info("PARITY harness armed: seed=" + paritySeed
                    + " seeded=" + seeded + " lvlSeeded=" + lvlSeeded
                    + " fillStartTick=" + parityFillStartTick + " (bot ignored)");
            Bukkit.getScheduler().runTaskTimer(this, () -> {
                if (!started && tickNo >= parityFillStartTick) {
                    started = true; // ONE-SHOT — begin() must never run twice
                    // Re-pin the WORLD random at fill-start: boot-phase draws
                    // (spawn-point search, worldgen edges) may consume a
                    // variable count of Level.random — a fresh seed here makes
                    // the fill window's starting state seed-exact regardless.
                    boolean lvl2 = seedLevelRandom("fill-start");
                    getLogger().info("PARITY: fixed-tick fill start at T" + tickNo
                            + " lvlSeeded=" + lvl2);
                    begin(null);
                }
            }, 1L, 1L);
        }
    }

    private static long longEnv(String k, long d) {
        try { return Long.parseLong(env(k, String.valueOf(d))); } catch (Exception e) { return d; }
    }

    // ------------------------------------------------------------------
    // Paper's entities share ONE RandomSource (Entity.SHARED_RANDOM,
    // Entity$RandomRandomSource — a package-private inner class whose
    // setSeed is `public synchronized` but INVOKABLE only via
    // setAccessible). The source LOCKS on its first setSeed — later seeds
    // are logged and ignored — so this must run exactly once, before any
    // entity exists. Returns whether the seed was actually applied.
    // ------------------------------------------------------------------
    private boolean seedSharedRandom(String where) {
        try {
            if (!sharedRandomTried) {
                sharedRandomTried = true;
                Class<?> c = Class.forName("net.minecraft.world.entity.Entity", false, getClass().getClassLoader());
                Field f = c.getField("SHARED_RANDOM");
                sharedRandom = f.get(null);
            }
            if (sharedRandom == null) return false;
            java.lang.reflect.Method m = sharedRandom.getClass().getMethod("setSeed", long.class);
            m.setAccessible(true); // the declaring class is package-private
            m.invoke(sharedRandom, paritySeed);
            return true;
        } catch (Throwable t) {
            getLogger().warning("PARITY: cannot seed SHARED_RANDOM (" + where + "): " + t);
            return false;
        }
    }

    // ------------------------------------------------------------------
    // Level.random (protected final RandomSource on Level) is a SECOND
    // entropy stream the old harness never pinned. Mob.finalizeSpawn —
    // invoked by CraftRegionAccessor.addEntity on EVERY Bukkit-spawned
    // mob (randomizeData=true) — draws its canPickUpLoot / baby-odds /
    // CHICKEN-JOCKEY / canBreakDoors / equipment rolls from
    // `level.getRandom()` (Mob.java:1201, 26.3), NOT from
    // SHARED_RANDOM. Unpinned, the two legs rolled different jockeys:
    // the vanilla off-leg itself varied across CI runs (n_off=2505/2506/
    // 2508 at mobs-T382) and every knob config "failed" with ~10-12
    // CHICKEN<->ZOMBIE flips that were NEVER the module's fault.
    // LegacyRandomSource.setSeed is freely re-callable (no lock, unlike
    // SHARED_RANDOM) — pin it once at enable and once at fill-start
    // (a fill-start re-pin also absorbs any variable boot-phase draw
    // count, making the fill window's starting state seed-exact).
    // ------------------------------------------------------------------
    private java.lang.reflect.Method lvlSeedM;
    private Object lvlRandom;
    private boolean lvlRandomTried;

    private boolean seedLevelRandom(String where) {
        try {
            if (!lvlRandomTried) {
                lvlRandomTried = true;
                World w = Bukkit.getWorlds().get(0);
                Object sl = w.getClass().getMethod("getHandle").invoke(w);
                Field f = null;
                for (Class<?> c = sl.getClass(); c != null && f == null; c = c.getSuperclass()) {
                    try {
                        f = c.getDeclaredField("random");
                    } catch (NoSuchFieldException ignore) { }
                }
                if (f == null) return false;
                f.setAccessible(true);
                lvlRandom = f.get(sl);
                if (lvlRandom == null) return false;
                lvlSeedM = lvlRandom.getClass().getMethod("setSeed", long.class);
                lvlSeedM.setAccessible(true);
            }
            if (lvlRandom == null || lvlSeedM == null) return false;
            lvlSeedM.invoke(lvlRandom, paritySeed ^ 0x9E3779B97F4A7C15L);
            return true;
        } catch (Throwable t) {
            getLogger().warning("PARITY: cannot seed Level.random (" + where + "): " + t);
            return false;
        }
    }

    private static String env(String k, String d) {
        String v = System.getenv(k);
        return v == null || v.isEmpty() ? d : v;
    }

    private static int intEnv(String k, int d) {
        try { return Integer.parseInt(env(k, String.valueOf(d))); } catch (Exception e) { return d; }
    }

    private static double doubleEnv(String k, double d) {
        try { return Double.parseDouble(env(k, String.valueOf(d))); } catch (Exception e) { return d; }
    }

    @EventHandler
    public void onJoin(PlayerJoinEvent e) {
        if (started) return;
        started = true;
        getLogger().info("player joined: " + e.getPlayer().getName() + " — starting fill");
        begin(e.getPlayer().getLocation());
    }

    private void begin(Location a) {
        World w = Bukkit.getWorlds().get(0);
        this.anchor = a != null ? a.clone() : w.getSpawnLocation().clone();
        this.groundY = w.getHighestBlockYAt(anchor.getBlockX(), anchor.getBlockZ()) + 1;

        // Plugin chunk tickets (level 31 => entity ticking) over both regions.
        int r = (Math.max(area, tileArea) / 16) + 2;
        int bcx = anchor.getBlockX() >> 4, bcz = anchor.getBlockZ() >> 4;
        for (int dx = -r; dx <= r; dx++) {
            for (int dz = -r; dz <= r; dz++) {
                w.addPluginChunkTicket(bcx + dx, bcz + dz, this);
            }
        }

        // Queue tile columns: chest(top,items) / hopper / chest(bottom);
        // furnace columns: furnace only (fuel+input set after placement).
        int half = tileArea / 2;
        int side = (int) Math.ceil(Math.sqrt(Math.max(1, tileHoppers + tileFurnaces)));
        int i = 0;
        for (int gx = -side / 2; gx < side / 2 && i < tileHoppers + tileFurnaces; gx++) {
            for (int gz = -side / 2; gz < side / 2 && i < tileHoppers + tileFurnaces; gz++, i++) {
                int x = anchor.getBlockX() + gx, z = anchor.getBlockZ() + gz;
                if (Math.abs(gx) > half || Math.abs(gz) > half) continue;
                if (i < tileHoppers) {
                    placeQueue.add(new int[]{1, x, groundY + 1, z}); // bottom chest
                    placeQueue.add(new int[]{2, x, groundY + 2, z}); // hopper
                    placeQueue.add(new int[]{3, x, groundY + 3, z}); // top chest (items)
                } else {
                    placeQueue.add(new int[]{4, x, groundY + 1, z}); // furnace
                }
            }
        }

        phase = "FILL";
        Bukkit.getScheduler().runTaskTimer(this, this::fillStep, 40L, 1L);
    }

    private void fillStep() {
        World w = Bukkit.getWorlds().get(0);
        if (tickNo < 1) return;

        // --- entities ---
        if (spawnedItems + spawnedStands + spawnedZombies < scale) {
            int budget = (int) Math.ceil(scale / (double) fillTicks);
            double carryBak = entityCarry;
            while (budget > 0 && spawnedItems + spawnedStands + spawnedZombies < scale) {
                double pick = rnd.nextDouble();
                EntityType t = pick < pctItems ? EntityType.ITEM
                        : pick < pctItems + pctStands ? EntityType.ARMOR_STAND
                        : EntityType.ZOMBIE;
                int dx, dz;
                boolean innerZone = t == EntityType.ZOMBIE && rnd.nextDouble() < 0.5;
                if (innerZone) {
                    int r = Math.max(1, innerRadius);
                    dx = rnd.nextInt(2 * r + 1) - r;
                    dz = rnd.nextInt(2 * r + 1) - r;
                } else {
                    int r = area / 2;
                    dx = rnd.nextInt(2 * r + 1) - r;
                    dz = rnd.nextInt(2 * r + 1) - r;
                }
                Location loc = new Location(w, anchor.getX() + dx + 0.5,
                        groundY + 1.0, anchor.getZ() + dz + 0.5);
                Entity en = w.spawnEntity(loc, t);
                if (en == null) { entityCarry = carryBak; break; }
                if (t == EntityType.ITEM) {
                    Item it = (Item) en;
                    it.setItemStack(new ItemStack(Material.STONE, 1));
                    if (rnd.nextDouble() < pctMergable) it.setPickupDelay(2);
                    else it.setPickupDelay(32767);
                    it.setCanMobPickup(false);
                    spawnedItems++;
                } else if (t == EntityType.ARMOR_STAND) {
                    ArmorStand st = (ArmorStand) en;
                    st.setMarker(true);
                    st.setGravity(false);
                    spawnedStands++;
                } else {
                    Zombie zb = (Zombie) en;
                    zb.setRemoveWhenFarAway(false);
                    ItemStack helm = new ItemStack(Material.IRON_HELMET);
                    helm.addUnsafeEnchantment(Enchantment.UNBREAKING, 255);
                    zb.getEquipment().setHelmet(helm);
                    spawnedZombies++;
                }
                budget--;
            }
        }

        // --- tiles ---
        if (!placeQueue.isEmpty()) {
            int budget = (int) Math.ceil((tileHoppers * 3 + tileFurnaces) / (double) fillTicks);
            while (budget > 0 && !placeQueue.isEmpty()) {
                int[] job = placeQueue.peek();
                int cx = job[1] >> 4, cz = job[3] >> 4;
                if (!w.isChunkLoaded(cx, cz)) break; // retry next tick
                placeQueue.poll();
                var block = w.getBlockAt(job[1], job[2], job[3]);
                switch (job[0]) {
                    case 1 -> block.setType(Material.CHEST);
                    case 2 -> block.setType(Material.HOPPER);
                    case 3 -> {
                        block.setType(Material.CHEST);
                        Chest c = (Chest) block.getState();
                        c.getInventory().setItem(0, new ItemStack(Material.DIRT, 64));
                        c.getInventory().setItem(1, new ItemStack(Material.DIRT, 64));
                        c.getInventory().setItem(2, new ItemStack(Material.DIRT, 64));
                        placedHoppers++;
                    }
                    case 4 -> {
                        block.setType(Material.FURNACE);
                        Furnace f = (Furnace) block.getState();
                        f.setBurnTime((short) 1600);
                        f.getInventory().setFuel(new ItemStack(Material.COAL, 64));
                        f.getInventory().setSmelting(new ItemStack(Material.IRON_ORE, 64));
                        f.update();
                        placedFurnaces++;
                    }
                    default -> { }
                }
                budget--;
            }
        }

        boolean entDone = spawnedItems + spawnedStands + spawnedZombies >= scale;
        boolean tileDone = placeQueue.isEmpty();
        if (entDone && tileDone && "FILL".equals(phase)) {
            phase = "WARMUP";
            measureEndTick = tickNo + warmupTicks + measureTicks;
            if (parity) {
                fillDoneTick = tickNo;
                getLogger().info("PARITY: fillDoneTick=" + fillDoneTick
                        + " — RNG stream free-runs from the single onEnable seed");
            }
            getLogger().info(String.format(Locale.ROOT,
                    "fill done: items=%d stands=%d zombies=%d hoppers=%d furnaces=%d — warmup %d ticks",
                    spawnedItems, spawnedStands, spawnedZombies, placedHoppers, placedFurnaces, warmupTicks));
        }
    }

    @EventHandler
    public void onTickEnd(ServerTickEndEvent e) {
        tickNo++;
        // Parity snapshots at fill-done offsets (mid-warmup, mid-measure) —
        // resolved in onTickEnd so the schedule works in EVERY phase
        // (offset 300 lands past the warmup->measure transition).
        if (parity && fillDoneTick > 0) {
            for (int off : PARITY_SNAP_OFFSETS) {
                if (tickNo == fillDoneTick + off) {
                    writeParitySnapshot("T" + tickNo, off);
                }
            }
        }
        if (parity) {
            logParityHash();
        }
        if (parity && traceId >= 0) {
            traceStep();
        }
        switch (phase) {
            case "WARMUP" -> {
                if (tickNo >= measureEndTick - measureTicks) {
                    phase = "MEASURE";
                    getLogger().info("measure window open");
                    String hs = hookStats();
                    if (hs != null) getLogger().info("hooks@measure-open: " + hs);
                }
            }
            case "MEASURE" -> {
                tickDurations.add(new double[]{tickNo, e.getTickDuration()});
                if (tickNo % 200 == 0) {
                    double heapUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1048576.0;
                    // L8 parity telemetry: item MASS (sum of stack counts) + live
                    // item count. Mass conservation is the vanilla-compat gate:
                    // entity counts can diverge legitimately (merge order), mass
                    // cannot — a drift here means duplication or loss.
                    double itemMass = 0.0, itemCount = 0.0;
                    for (org.bukkit.entity.Entity en : Bukkit.getWorlds().get(0).getEntities()) {
                        if (en instanceof org.bukkit.entity.Item it) {
                            itemCount += 1.0;
                            itemMass += it.getItemStack().getAmount();
                        }
                    }
                    samples.add(new double[]{tickNo, Bukkit.getWorlds().get(0).getEntities().size(),
                            placedHoppers, placedFurnaces, heapUsed, itemMass, itemCount});
                    String hs = hookStats();
                    if (hs != null) getLogger().info("hooks@" + tickNo + ": " + hs);
                }
                if (tickDurations.size() >= measureTicks) finish();
            }
            default -> { }
        }
    }

    // ------------------------------------------------------------------
    // c-entities hook counters (diagnostics). The CeHooks class lives in the
    // server/kernel classloader, not in this plugin's — resolve it through
    // this classloader's parents and read the public static fields via
    // reflection. Returns null when the module is absent (vanilla runs).
    // ------------------------------------------------------------------
    private Class<?> ceHooksClass;
    private boolean ceHooksTried;

    private Class<?> ceHooks() {
        if (ceHooksTried) return ceHooksClass;
        ceHooksTried = true;
        ClassLoader cl = getClass().getClassLoader();
        for (int depth = 0; cl != null && depth < 8; depth++, cl = cl.getParent()) {
            try {
                ceHooksClass = Class.forName("dev.planeta9091.centities.CeHooks", false, cl);
                return ceHooksClass;
            } catch (ClassNotFoundException ignore) { }
        }
        // last resort: context loader
        ClassLoader ctx = Thread.currentThread().getContextClassLoader();
        if (ctx != null) {
            try { ceHooksClass = Class.forName("dev.planeta9091.centities.CeHooks", false, ctx); }
            catch (ClassNotFoundException ignore) { }
        }
        return ceHooksClass;
    }

    private String hookStats() {
        Class<?> c = ceHooks();
        if (c == null) return null;
        try {
            Object r = c.getMethod("stats").invoke(null);
            return String.valueOf(r);
        } catch (Throwable t) {
            return "stats() failed: " + t;
        }
    }

    private long hookField(String name) {
        Class<?> c = ceHooks();
        if (c == null) return -1;
        try { return c.getField(name).getLong(null); }
        catch (Throwable t) { return -1; }
    }

    private static double pct(double[] sorted, double p) {
        if (sorted.length == 0) return 0;
        int i = (int) Math.min(sorted.length - 1, Math.max(0, (int) Math.round(p * (sorted.length - 1))));
        return sorted[i];
    }

    private void finish() {
        phase = "DONE";
        if (parity) writeParitySnapshot("final", -1);
        double[] ds = new double[tickDurations.size()];
        for (int i = 0; i < ds.length; i++) ds[i] = tickDurations.get(i)[1];
        Arrays.sort(ds);
        double mean = 0;
        for (double d : ds) mean += d;
        mean = ds.length == 0 ? 0 : mean / ds.length;
        double heapMax = Runtime.getRuntime().maxMemory() / 1048576.0;
        double heapUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1048576.0;

        StringBuilder j = new StringBuilder(4096);
        j.append("{\n");
        j.append("  \"run\": \"").append(runName).append("\",\n");
        j.append("  \"server\": \"").append(Bukkit.getVersion().replace("\"", "'")).append("\",\n");
        j.append("  \"config\": {\"scale\": ").append(scale)
                .append(", \"fill_ticks\": ").append(fillTicks)
                .append(", \"warmup\": ").append(warmupTicks)
                .append(", \"measure\": ").append(measureTicks)
                .append(", \"area\": ").append(area)
                .append(", \"inner_radius\": ").append(innerRadius)
                .append(", \"pct_items\": ").append(pctItems)
                .append(", \"pct_stands\": ").append(pctStands)
                .append(", \"pct_mergable\": ").append(pctMergable)
                .append(", \"tile_hoppers\": ").append(tileHoppers)
                .append(", \"tile_furnaces\": ").append(tileFurnaces).append("},\n");
        j.append("  \"spawned\": {\"items\": ").append(spawnedItems)
                .append(", \"stands\": ").append(spawnedStands)
                .append(", \"zombies\": ").append(spawnedZombies)
                .append(", \"hoppers\": ").append(placedHoppers)
                .append(", \"furnaces\": ").append(placedFurnaces).append("},\n");
        j.append("  \"mspt\": {\"mean\": ").append(String.format(Locale.ROOT, "%.3f", mean))
                .append(", \"p50\": ").append(String.format(Locale.ROOT, "%.3f", pct(ds, 0.50)))
                .append(", \"p90\": ").append(String.format(Locale.ROOT, "%.3f", pct(ds, 0.90)))
                .append(", \"p99\": ").append(String.format(Locale.ROOT, "%.3f", pct(ds, 0.99)))
                .append(", \"max\": ").append(String.format(Locale.ROOT, "%.3f", pct(ds, 1.0)))
                .append(", \"n\": ").append(ds.length).append("},\n");
        j.append("  \"heap_mb\": {\"used\": ").append(String.format(Locale.ROOT, "%.1f", heapUsed))
                .append(", \"max\": ").append(String.format(Locale.ROOT, "%.1f", heapMax)).append("},\n");
        if (ceHooks() != null) {
            j.append("  \"hooks\": {\"fast\": ").append(hookField("queriesFast"))
                    .append(", \"mirror\": ").append(hookField("queriesMirror"))
                    .append(", \"notServerLevel\": ").append(hookField("mNotServerLevel"))
                    .append(", \"badWorld\": ").append(hookField("mBadWorld"))
                    .append(", \"notReady\": ").append(hookField("mNotReady"))
                    .append(", \"regrowFail\": ").append(hookField("mRegrowFail"))
                    .append(", \"desync\": ").append(hookField("mDesync"))
                    .append(", \"disabled\": ").append(hookField("mDisabled"))
                    .append(", \"probesAdd\": ").append(hookField("probesAdd"))
                    .append(", \"probesRemove\": ").append(hookField("probesRemove"))
                    .append(", \"probesBox\": ").append(hookField("probesBox"))
                    .append(", \"batchesFlushed\": ").append(hookField("batchesFlushed"))
                    .append(", \"batchEntries\": ").append(hookField("batchEntries"))
                    .append("},\n");
        }
        j.append("  \"samples\": [\n");
        for (int i = 0; i < samples.size(); i++) {
            double[] s = samples.get(i);
            j.append("    {\"tick\": ").append((long) s[0])
                    .append(", \"entities\": ").append((long) s[1])
                    .append(", \"hoppers\": ").append((long) s[2])
                    .append(", \"furnaces\": ").append((long) s[3])
                    .append(", \"heap_mb\": ").append(String.format(Locale.ROOT, "%.1f", s[4]))
                    .append(", \"item_mass\": ").append((long) s[5])
                    .append(", \"item_count\": ").append((long) s[6])
                    .append("}").append(i + 1 < samples.size() ? "," : "").append('\n');
        }
        j.append("  ],\n  \"tick_ms\": [");
        for (int i = 0; i < tickDurations.size(); i++) {
            if (i > 0 && i % 20 == 0) j.append("\n   ");
            j.append(String.format(Locale.ROOT, "%.2f", tickDurations.get(i)[1]));
            if (i + 1 < tickDurations.size()) j.append(',');
        }
        j.append("]\n}\n");

        try {
            File out = new File(Bukkit.getWorldContainer(), "baseline.json");
            Files.writeString(out.toPath(), j.toString(), StandardCharsets.UTF_8);
            getLogger().info("BASELINE-DONE " + out.getAbsolutePath());
        } catch (Exception ex) {
            getLogger().severe("failed to write baseline.json: " + ex);
        }
        Bukkit.getScheduler().runTaskLater(this, Bukkit::shutdown, 40L);
    }

    // ------------------------------------------------------------------
    // Behavioral parity snapshot: the FULL observable entity state, written
    // with EXACT bit patterns (hex floats/doubles — no decimal rounding),
    // sorted by entity id. Two servers (module off/on) that behave
    // identically MUST produce identical files; sha256 of each snapshot is
    // logged as PARITY-SNAP for a quick CI grep gate.
    // ------------------------------------------------------------------
    private void writeParitySnapshot(String tag, int offset) {
        if (!snapDone.add((long) tag.hashCode())) return;
        try {
            // L37 diagnostics: the activation/tick bookkeeping is the most
            // order-sensitive state two legs can share — divergences in
            // tickCount/activatedTick pinpoint WHERE the streams split
            // (activation windows, dispatcher phase) long before positions
            // drift visibly. Parity mode only; perf is irrelevant here.
            // getHandle lives on CraftEntity (CraftBukkit), NOT on the
            // Bukkit Entity interface — bind it from the first entity's
            // RUNTIME class or every snapshot dies NoSuchMethodException
            // (observed 2026-10-27: zero parity files, gate exit 2).
            List<org.bukkit.entity.Entity> ents = new ArrayList<>(Bukkit.getWorlds().get(0).getEntities());
            if (!ents.isEmpty() && PARITY_GET_HANDLE == null) {
                synchronized (BaselinePlugin.class) {
                    if (PARITY_GET_HANDLE == null) {
                        PARITY_GET_HANDLE = ents.get(0).getClass()
                                .getMethod("getHandle");
                        Class<?> nms = PARITY_GET_HANDLE.getReturnType();
                        PARITY_TICK_COUNT = nms.getField("tickCount");
                        PARITY_ACTIVATED = nms.getField("activatedTick");
                    }
                }
            }
            ents.sort(Comparator.comparingInt(org.bukkit.entity.Entity::getEntityId));
            StringBuilder sb = new StringBuilder(ents.size() * 160 + 128);
            sb.append("{\"tag\":\"").append(tag).append("\",\"tick\":").append(tickNo)
                    .append(",\"seed\":").append(paritySeed)
                    .append(",\"fillDone\":").append(fillDoneTick)
                    .append(",\"n\":").append(ents.size())
                    .append(",\"entities\":[\n");
            for (int i = 0; i < ents.size(); i++) {
                org.bukkit.entity.Entity en = ents.get(i);
                Location l = en.getLocation();
                org.bukkit.util.Vector v = en.getVelocity();
                sb.append("{\"id\":").append(en.getEntityId())
                        .append(",\"t\":\"").append(en.getType().name()).append('\"')
                        .append(",\"x\":").append(hex(l.getX()))
                        .append(",\"y\":").append(hex(l.getY()))
                        .append(",\"z\":").append(hex(l.getZ()))
                        .append(",\"yaw\":").append(hexf(l.getYaw()))
                        .append(",\"pitch\":").append(hexf(l.getPitch()))
                        .append(",\"vx\":").append(hex(v.getX()))
                        .append(",\"vy\":").append(hex(v.getY()))
                        .append(",\"vz\":").append(hex(v.getZ()))
                        .append(",\"onG\":").append(en.isOnGround() ? 1 : 0)
                        .append(",\"fire\":").append(en.getFireTicks())
                        .append(",\"alive\":").append(en.isValid() ? 1 : 0);
                try {
                    Object h = PARITY_GET_HANDLE.invoke(en);
                    sb.append(",\"tc\":").append(PARITY_TICK_COUNT.getInt(h))
                            .append(",\"at\":").append(PARITY_ACTIVATED.getLong(h));
                } catch (Throwable quiet) {
                    // L37 diagnostics only — never break the snapshot
                }
                if (en instanceof org.bukkit.entity.LivingEntity le) {
                    sb.append(",\"hp\":").append(hex(le.getHealth()))
                            .append(",\"hurtT\":").append(le.getNoActionTicks());
                }
                if (en instanceof org.bukkit.entity.Item it) {
                    sb.append(",\"amt\":").append(it.getItemStack().getAmount())
                            .append(",\"pd\":").append(it.getPickupDelay());
                }
                sb.append('}').append(i + 1 < ents.size() ? "," : "").append('\n');
            }
            sb.append("]}\n");
            byte[] bytes = sb.toString().getBytes(StandardCharsets.UTF_8);
            File out = new File(Bukkit.getWorldContainer(), "parity-" + tag + ".json");
            Files.write(out.toPath(), bytes);
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            StringBuilder hx = new StringBuilder(64);
            for (byte b : md.digest(bytes)) hx.append(String.format("%02x", b));
            getLogger().info("PARITY-SNAP tag=" + tag + (offset >= 0 ? " off=" + offset : "")
                    + " n=" + ents.size() + " sha256=" + hx);
        } catch (Exception ex) {
            getLogger().severe("PARITY: snapshot " + tag + " failed: " + ex);
        }
    }

    private static String hex(double d) {
        return "\"" + Double.toHexString(d) + "\"";
    }

    private static String hexf(float f) {
        return "\"" + Float.toHexString(f) + "\"";
    }

    // L37 parity diagnostics: lazily-bound NMS accessors for tickCount /
    // activatedTick (parity mode only — perf irrelevant there).
    private static volatile java.lang.reflect.Method PARITY_GET_HANDLE;
    private static volatile java.lang.reflect.Field PARITY_TICK_COUNT, PARITY_ACTIVATED;

    // ------------------------------------------------------------------
    // PARITY PHASH: per-tick ORDER-INDEPENDENT checksum of the whole
    // observable entity state. One "PH t=.. h=.. n=.. adds=.." line per
    // tick in each leg's log (parity mode only); a post-hoc diff of the
    // two legs' log streams names the FIRST diverging tick and scenario —
    // the snapshot triple only shows T+50/+300/+650 accumulations, which
    // is useless for locating the seed of a cascade (observed: mixed has
    // 3814 type flips by T389 — the seed is ticks earlier, invisible in
    // snapshots). Order-independent by construction: commutative sum of a
    // per-entity 64-bit avalanche, so the world's entity-list iteration
    // order can neither fake nor mask a divergence. Spawn divergence is
    // visible through adds= (entity ids are monotonic within a session,
    // so id > watermark <=> new entity); removals are derivable post-hoc
    // (n_prev - n + adds). Zero allocation: MethodHandles with primitive
    // signatures (no boxing), ~8 invokes + 7 multiplies per entity.
    // ------------------------------------------------------------------
    private static volatile java.lang.invoke.MethodHandle PH_GET_HANDLE,
            PH_X, PH_Y, PH_Z, PH_DM, PH_VX, PH_VY, PH_VZ;
    private long phMaxId;
    private long phLines;

    private static long phMix(long k) {
        k *= 0x9E3779B97F4A7C15L;
        k ^= k >>> 30; k *= 0xBF58476D1CE4E5B9L;
        k ^= k >>> 27; k *= 0x94D049BB133111EBL;
        return k ^ (k >>> 31);
    }

    private void phBind(Object nmsSample) {
        try {
            var lk = java.lang.invoke.MethodHandles.lookup();
            Class<?> nms = nmsSample.getClass();
            java.lang.reflect.Method m;
            m = nms.getMethod("getX"); PH_X = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
            m = nms.getMethod("getY"); PH_Y = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
            m = nms.getMethod("getZ"); PH_Z = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
            m = nms.getMethod("getDeltaMovement");
            PH_DM = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(Object.class, Object.class));
            Class<?> vec3 = m.getReturnType();
            m = vec3.getMethod("getX"); PH_VX = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
            m = vec3.getMethod("getY"); PH_VY = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
            m = vec3.getMethod("getZ"); PH_VZ = lk.unreflect(m)
                    .asType(java.lang.invoke.MethodType.methodType(double.class, Object.class));
        } catch (Throwable t) {
            getLogger().warning("PH bind failed (n/adds only): " + t);
        }
    }

    private void logParityHash() {
        try {
            List<Entity> ents = Bukkit.getWorlds().get(0).getEntities();
            int n = ents.size();
            // seed the sum with the tick so a symmetric remove+respawn in
            // one tick cannot produce a constant stream
            long h = phMix(tickNo);
            long adds = 0, maxId = phMaxId;
            if (ents.isEmpty()) {
                phMaxId = maxId;
                getLogger().info("PH t=" + tickNo + " h=" + String.format("%016x", h)
                        + " n=0 adds=0");
                return;
            }
            if (PH_GET_HANDLE == null) {
                phBindRoot(ents.get(0));
            }
            boolean bound = PH_GET_HANDLE != null;
            if (bound && PH_X == null) {
                Object sample = PH_GET_HANDLE.invoke(ents.get(0));
                if (sample != null) phBind(sample);
                bound = PH_X != null;
            }
            for (int i = 0; i < n; i++) {
                Entity be = ents.get(i);
                int id = be.getEntityId();
                if (id > maxId) { adds++; maxId = id; }
                if (!bound) continue;
                Object e = PH_GET_HANDLE.invoke(be);
                double x = (double) PH_X.invokeExact(e);
                double y = (double) PH_Y.invokeExact(e);
                double z = (double) PH_Z.invokeExact(e);
                Object dm = (Object) PH_DM.invokeExact(e);
                long m = phMix(id);
                m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(x)), 7);
                m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(y)), 17);
                m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(z)), 27);
                if (dm != null) {
                    double vx = (double) PH_VX.invokeExact(dm);
                    double vy = (double) PH_VY.invokeExact(dm);
                    double vz = (double) PH_VZ.invokeExact(dm);
                    m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(vx)), 37);
                    m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(vy)), 47);
                    m ^= Long.rotateLeft(phMix(Double.doubleToLongBits(vz)), 57);
                }
                h += phMix(m);
            }
            phMaxId = maxId;
            phLines++;
            // Adaptive emit: full per-tick resolution through t=200 (the seed
            // zone — observed: mixed ids 9/10/45 vanish by T389), 1-in-8
            // afterwards (the cascade is already lost there; the ±8-tick
            // window is enough to aim a PARITY_TRACE_ID run). Keeps the
            // streamed job log at ~270 lines/leg instead of ~72k.
            if (tickNo <= 200 || tickNo % 8 == 0) {
                getLogger().info("PH t=" + tickNo + " h=" + String.format("%016x", h)
                        + " n=" + n + " adds=" + adds);
            }
        } catch (Throwable t) {
            if (phLines < 3) getLogger().warning("PH step failed: " + t);
        }
    }

    private void phBindRoot(Entity be) {
        try {
            var lk = java.lang.invoke.MethodHandles.lookup();
            java.lang.reflect.Method gh = be.getClass().getMethod("getHandle");
            PH_GET_HANDLE = lk.unreflect(gh)
                    .asType(java.lang.invoke.MethodType.methodType(Object.class, Object.class));
        } catch (Throwable t) {
            getLogger().warning("PH root bind failed: " + t);
        }
    }

    // ------------------------------------------------------------------
    // PARITY_TRACE_ID: per-tick EXACT trace of ONE entity (both legs write
    // parity-trace.jsonl; a line diff names the diverging TICK and FIELD —
    // the snapshot triple only shows the ACCUMULATED state).
    // ------------------------------------------------------------------
    private int traceId;
    private org.bukkit.entity.Entity traced;
    private java.io.Writer traceOut;

    private void traceStep() {
        try {
            if (traceOut == null) {
                traceOut = new java.io.OutputStreamWriter(
                        new java.io.FileOutputStream(
                                new File(Bukkit.getWorldContainer(), "parity-trace.jsonl"),
                                true),
                        StandardCharsets.UTF_8);
            }
            if (traced == null) {
                for (org.bukkit.entity.Entity en : Bukkit.getWorlds().get(0).getEntities()) {
                    if (en.getEntityId() == traceId) {
                        traced = en;
                        break;
                    }
                }
                if (traced == null) {
                    traceOut.write("t=" + tickNo + " pending\n");
                    traceOut.flush();
                    return;
                }
                // bind the NMS accessors NOW (not at the first snapshot) —
                // the tc/at columns must cover the trace from tick one (the
                // item's %4 phase diagnosis needs tc at the diverging tick)
                if (PARITY_GET_HANDLE == null) {
                    synchronized (BaselinePlugin.class) {
                        if (PARITY_GET_HANDLE == null) {
                            PARITY_GET_HANDLE = traced.getClass().getMethod("getHandle");
                            Class<?> nms = PARITY_GET_HANDLE.getReturnType();
                            PARITY_TICK_COUNT = nms.getField("tickCount");
                            PARITY_ACTIVATED = nms.getField("activatedTick");
                        }
                    }
                }
            }
            if (!traced.isValid()) {
                traceOut.write("t=" + tickNo + " REMOVED\n");
                traceOut.flush();
                return;
            }
            Location l = traced.getLocation();
            org.bukkit.util.Vector v = traced.getVelocity();
            StringBuilder sb = new StringBuilder(160);
            sb.append("t=").append(tickNo)
                    .append(" type=").append(traced.getType().name());
            if (PARITY_GET_HANDLE != null) {
                try {
                    Object h = PARITY_GET_HANDLE.invoke(traced);
                    sb.append(" tc=").append(PARITY_TICK_COUNT.getInt(h))
                            .append(" at=").append(PARITY_ACTIVATED.getLong(h));
                } catch (Throwable quiet) {
                    // diagnostics only
                }
            }
            sb.append(" x=").append(hex(l.getX()))
                    .append(" y=").append(hex(l.getY()))
                    .append(" z=").append(hex(l.getZ()))
                    .append(" yaw=").append(hexf(l.getYaw()))
                    .append(" vx=").append(hex(v.getX()))
                    .append(" vy=").append(hex(v.getY()))
                    .append(" vz=").append(hex(v.getZ()))
                    .append(" onG=").append(traced.isOnGround() ? 1 : 0)
                    .append('\n');
            traceOut.write(sb.toString());
            traceOut.flush();
        } catch (Throwable t) {
            getLogger().warning("trace step failed: " + t);
        }
    }
}
