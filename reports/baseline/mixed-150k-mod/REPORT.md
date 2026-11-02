# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109710 -> 109710) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 267.246 / 377.106 / 567.209 / 675.695 |
| MSPT mean, ms | 295.913 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1961.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 3.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10099` |
| 5 | 2.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9497` |
| 8 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5454` |
| 10 | 1.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

