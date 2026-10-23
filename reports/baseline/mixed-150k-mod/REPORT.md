# Baseline run: `mixed-150k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 99894 -> 99867) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 341.605 / 397.292 / 450.030 / 513.076 |
| MSPT mean, ms | 351.615 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2365.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 5.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 3.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 3.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 1.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

