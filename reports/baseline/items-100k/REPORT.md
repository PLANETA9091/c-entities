# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62977 -> 62977) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.826 / 78.827 / 144.812 / 201.658 |
| MSPT mean, ms | 71.027 |
| TPS estimate (1000/p50) | 15.4 |
| heap used / max, MB | 841.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.87% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 9.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 5.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 3.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.05% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.59% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 14 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.20% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

