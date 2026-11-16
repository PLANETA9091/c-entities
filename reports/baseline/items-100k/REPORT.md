# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62936 -> 62936) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 71.041 / 114.887 / 196.151 / 370.357 |
| MSPT mean, ms | 82.845 |
| TPS estimate (1000/p50) | 14.1 |
| heap used / max, MB | 988.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.21% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

