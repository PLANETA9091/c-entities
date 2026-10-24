# Baseline run: `items-100k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62983 -> 62983) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.839 / 72.604 / 132.030 / 193.093 |
| MSPT mean, ms | 65.042 |
| TPS estimate (1000/p50) | 16.7 |
| heap used / max, MB | 1248.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.56% | `java.util.ArrayList$Itr.<init>(ArrayList) line: 1042` |
| 2 | 7.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 4.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.59% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 15 | 1.35% | `net.minecraft.core.Vec3i.getY() line: 67` |

