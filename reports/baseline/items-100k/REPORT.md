# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62881 -> 62881) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.353 / 78.745 / 145.797 / 217.316 |
| MSPT mean, ms | 69.609 |
| TPS estimate (1000/p50) | 15.8 |
| heap used / max, MB | 1106.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.42% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 9.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.83% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 1.79% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.27% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

