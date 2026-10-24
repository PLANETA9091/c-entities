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
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.List;
import java.util.Locale;
import java.util.Random;

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

    // CE_DIFF=1: differential correctness test (index fast path vs vanilla
    // mirror) — runs during WARMUP only, so the MEASURE window stays clean.
    private boolean ceDiff = false;
    private long diffChecks = 0, diffMismatch = 0;
    private Class<?> ceHooksCls = null;
    private java.lang.reflect.Field ceActive, ceFast, ceMirror;

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

        // Neutralize non-entity noise.
        for (World w : Bukkit.getWorlds()) {
            w.setGameRule(GameRule.DO_MOB_SPAWNING, false);
            w.setGameRule(GameRule.DO_DAYLIGHT_CYCLE, false);
            w.setGameRule(GameRule.DO_FIRE_TICK, false);
            w.setGameRule(GameRule.MOB_GRIEFING, false);
            w.setGameRule(GameRule.RANDOM_TICK_SPEED, 0);
            w.setStorm(false);
            w.setThundering(false);
            w.setTime(6000L);
        }

        getServer().getPluginManager().registerEvents(this, this);
        getLogger().info(String.format(Locale.ROOT,
                "ce-baseline config: run=%s scale=%d items=%.2f stands=%.2f tiles(h=%d,f=%d) measure=%d",
                runName, scale, pctItems, pctStands, tileHoppers, tileFurnaces, measureTicks));

        ceDiff = "1".equals(System.getenv("CE_DIFF"));
        if (ceDiff) {
            try {
                ceHooksCls = Class.forName("dev.planeta9091.centities.CeHooks");
                ceActive = ceHooksCls.getField("ACTIVE");
                ceFast = ceHooksCls.getField("queriesFast");
                ceMirror = ceHooksCls.getField("queriesMirror");
                getLogger().info("ce-diff: CeHooks present (module loaded)");
            } catch (Throwable t) {
                getLogger().info("ce-diff: CeHooks absent (vanilla run) — plumbing check only");
            }
        }

        // Failsafe: if the fake player never joins, start anyway (marked no-player).
        Bukkit.getScheduler().runTaskLater(this, () -> {
            if (!started) {
                getLogger().warning("no player joined within 45s — starting without player (mobs will be inactive)");
                begin(null);
            }
        }, 20L * 45L);
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
            getLogger().info(String.format(Locale.ROOT,
                    "fill done: items=%d stands=%d zombies=%d hoppers=%d furnaces=%d — warmup %d ticks",
                    spawnedItems, spawnedStands, spawnedZombies, placedHoppers, placedFurnaces, warmupTicks));
        }
    }

    @EventHandler
    public void onTickEnd(ServerTickEndEvent e) {
        tickNo++;
        switch (phase) {
            case "WARMUP" -> {
                if (ceDiff && tickNo % 5 == 0) runDiffBatch(8);
                if (tickNo >= measureEndTick - measureTicks) {
                    phase = "MEASURE";
                    getLogger().info(String.format(Locale.ROOT,
                            "measure window open (diff: checks=%d mismatches=%d)", diffChecks, diffMismatch));
                }
            }
            case "MEASURE" -> {
                tickDurations.add(new double[]{tickNo, e.getTickDuration()});
                if (tickNo % 100 == 0) {
                    double heapUsed = (Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory()) / 1048576.0;
                    samples.add(new double[]{tickNo, Bukkit.getWorlds().get(0).getEntities().size(),
                            placedHoppers, placedFurnaces, heapUsed});
                }
                if (tickDurations.size() >= measureTicks) finish();
            }
            default -> { }
        }
    }

    private static double pct(double[] sorted, double p) {
        if (sorted.length == 0) return 0;
        int i = (int) Math.min(sorted.length - 1, Math.max(0, (int) Math.round(p * (sorted.length - 1))));
        return sorted[i];
    }

    // ------------------------------------------------------------------
    // Differential test: index fast path must return EXACTLY the vanilla
    // entity-id set for random query boxes (order-insensitive).
    // ------------------------------------------------------------------
    private void runDiffBatch(int n) {
        try {
            World w = Bukkit.getWorlds().get(0);
            net.minecraft.server.level.ServerLevel sl =
                    ((org.bukkit.craftbukkit.CraftWorld) w).getHandle();
            java.util.Random r = java.util.concurrent.ThreadLocalRandom.current();
            for (int i = 0; i < n; i++) {
                double x = anchor.getX() + (r.nextDouble() * 2 - 1) * innerRadius;
                double z = anchor.getZ() + (r.nextDouble() * 2 - 1) * innerRadius;
                double y = groundY - 2 + r.nextDouble() * 8;
                double s = 1.0 + r.nextDouble() * 8;
                net.minecraft.world.phys.AABB box = new net.minecraft.world.phys.AABB(
                        x - s / 2, y, z - s / 2, x + s / 2, y + 2 + r.nextDouble() * 2, z + s / 2);
                java.util.function.Predicate<net.minecraft.world.entity.Entity> pred =
                        e -> e.isAlive();

                // vanilla body of Level.getEntities(Entity, AABB, Predicate)
                java.util.List<net.minecraft.world.entity.Entity> vanilla = new java.util.ArrayList<>();
                sl.moonrise$getEntityLookup().getEntities(
                        (net.minecraft.world.entity.Entity) null, box, vanilla, pred);
                ca.spottedleaf.moonrise.common.PlatformHooks.get()
                        .addToGetEntities(sl, null, box, pred, vanilla);

                // woven path (fast when armed; bridge mirror otherwise)
                java.util.List<net.minecraft.world.entity.Entity> woven =
                        sl.getEntities((net.minecraft.world.entity.Entity) null, box, pred);

                diffChecks++;
                if (!sameIds(vanilla, woven)) {
                    diffMismatch++;
                    if (diffMismatch <= 3) getLogger().warning(String.format(Locale.ROOT,
                            "CE-DIFF entity mismatch: vanilla=%d woven=%d box=%s",
                            vanilla.size(), woven.size(), box));
                }

                // vanilla body of Level.getEntitiesOfClass(Class, AABB, Predicate)
                java.util.function.Predicate<net.minecraft.world.entity.item.ItemEntity> predI =
                        ie -> ie.isAlive();
                java.util.List<net.minecraft.world.entity.item.ItemEntity> vanillaI =
                        new java.util.ArrayList<>();
                sl.moonrise$getEntityLookup().getEntities(
                        net.minecraft.world.entity.item.ItemEntity.class, null, box, vanillaI, predI);
                java.util.List<net.minecraft.world.entity.item.ItemEntity> wovenI =
                        sl.getEntitiesOfClass(net.minecraft.world.entity.item.ItemEntity.class, box, predI);
                diffChecks++;
                if (!sameIds(vanillaI, wovenI)) {
                    diffMismatch++;
                    if (diffMismatch <= 3) getLogger().warning(String.format(Locale.ROOT,
                            "CE-DIFF item mismatch: vanilla=%d woven=%d box=%s",
                            vanillaI.size(), wovenI.size(), box));
                }
            }
        } catch (Throwable t) {
            getLogger().warning("ce-diff batch failed: " + t);
        }
    }

    private static boolean sameIds(java.util.List<?> a, java.util.List<?> b) {
        if (a.size() != b.size()) return false;
        long[] ia = new long[a.size()];
        for (int i = 0; i < ia.length; i++) ia[i] = ((net.minecraft.world.entity.Entity) a.get(i)).getId();
        long[] ib = new long[b.size()];
        for (int i = 0; i < ib.length; i++) ib[i] = ((net.minecraft.world.entity.Entity) b.get(i)).getId();
        Arrays.sort(ia);
        Arrays.sort(ib);
        return Arrays.equals(ia, ib);
    }

    private void finish() {
        phase = "DONE";
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
        j.append("  \"ce_diff\": {\"enabled\": ").append(ceDiff)
                .append(", \"checks\": ").append(diffChecks)
                .append(", \"mismatches\": ").append(diffMismatch).append("},\n");
        j.append("  \"ce_module\": {");
        if (ceHooksCls != null) {
            try {
                j.append("\"loaded\": true, \"active\": ").append(ceActive.getBoolean(null))
                        .append(", \"fast\": ").append(ceFast.getLong(null))
                        .append(", \"mirror\": ").append(ceMirror.getLong(null))
                        .append(", \"pos\": ").append(ceHooksCls.getField("probesPos").getLong(null))
                        .append(", \"posDedupHits\": ").append(ceHooksCls.getField("posDedupHits").getLong(null))
                        .append(", \"posNative\": ").append(ceHooksCls.getField("posNative").getLong(null))
                        .append(", \"fallback\": ").append(ceHooksCls.getField("queriesFallback").getLong(null));
            } catch (Throwable t) {
                j.append("\"loaded\": true, \"active\": false");
            }
        } else {
            j.append("\"loaded\": false");
        }
        j.append("},\n");
        j.append("  \"samples\": [\n");
        for (int i = 0; i < samples.size(); i++) {
            double[] s = samples.get(i);
            j.append("    {\"tick\": ").append((long) s[0])
                    .append(", \"entities\": ").append((long) s[1])
                    .append(", \"hoppers\": ").append((long) s[2])
                    .append(", \"furnaces\": ").append((long) s[3])
                    .append(", \"heap_mb\": ").append(String.format(Locale.ROOT, "%.1f", s[4]))
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
}
