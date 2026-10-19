# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62982 -> 62982) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.620 / 72.278 / 112.042 / 166.019 |
| MSPT mean, ms | 67.242 |
| TPS estimate (1000/p50) | 15.7 |
| heap used / max, MB | 1574.7 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 6 | 2.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.98% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 9 | 1.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 10 | 1.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 12 | 1.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 14 | 1.21% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 15 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |

