# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62947 -> 62946) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.840 / 106.230 / 169.188 / 197.910 |
| MSPT mean, ms | 74.887 |
| TPS estimate (1000/p50) | 15.7 |
| heap used / max, MB | 1111.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 2.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.72% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

