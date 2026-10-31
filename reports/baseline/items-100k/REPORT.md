# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62917 -> 62917) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.646 / 83.694 / 150.183 / 220.043 |
| MSPT mean, ms | 72.440 |
| TPS estimate (1000/p50) | 15.2 |
| heap used / max, MB | 933.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 2 | 7.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.87% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |

