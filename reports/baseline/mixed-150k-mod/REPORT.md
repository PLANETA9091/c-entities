# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100385 -> 100385) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 316.052 / 383.325 / 426.565 / 467.848 |
| MSPT mean, ms | 318.934 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2128.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.38% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 2.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 14 | 1.35% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.30% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |

