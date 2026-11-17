# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62998 -> 62997) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 88.701 / 160.257 / 260.106 / 313.580 |
| MSPT mean, ms | 102.088 |
| TPS estimate (1000/p50) | 11.3 |
| heap used / max, MB | 1391.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 2.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.70% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.53% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 14 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |

