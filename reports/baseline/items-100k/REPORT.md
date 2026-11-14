# Baseline run: `items-100k`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63011 -> 63012) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 60.032 / 102.413 / 150.373 / 188.113 |
| MSPT mean, ms | 70.852 |
| TPS estimate (1000/p50) | 16.7 |
| heap used / max, MB | 774.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 2.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.98% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.80% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 11 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.43% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

