# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.481 / 82.398 / 140.357 / 204.104 |
| MSPT mean, ms | 69.898 |
| TPS estimate (1000/p50) | 15.8 |
| heap used / max, MB | 1195.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 8.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.96% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 12 | 1.74% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.18% | `net.minecraft.core.Vec3i.getY() line: 67` |

