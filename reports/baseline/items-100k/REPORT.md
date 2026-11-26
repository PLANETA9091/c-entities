# Baseline run: `items-100k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62991 -> 62990) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.128 / 108.793 / 156.380 / 205.477 |
| MSPT mean, ms | 77.834 |
| TPS estimate (1000/p50) | 14.9 |
| heap used / max, MB | 868.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.62% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

