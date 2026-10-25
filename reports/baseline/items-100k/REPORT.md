# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62891 -> 62891) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.596 / 85.667 / 150.408 / 212.356 |
| MSPT mean, ms | 73.112 |
| TPS estimate (1000/p50) | 15.0 |
| heap used / max, MB | 803.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 7.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 3.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.25% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 8 | 2.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.96% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.12% | `net.minecraft.core.Vec3i.getY() line: 67` |

