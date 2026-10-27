# Baseline run: `items-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62937 -> 62937) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.359 / 74.014 / 139.111 / 168.444 |
| MSPT mean, ms | 62.870 |
| TPS estimate (1000/p50) | 17.4 |
| heap used / max, MB | 1431.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 5.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.76% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 5 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 1.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.77% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.64% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 11 | 1.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |

