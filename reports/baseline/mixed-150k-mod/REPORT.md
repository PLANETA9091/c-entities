# Baseline run: `mixed-150k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100339 -> 100339) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 182.589 / 331.784 / 391.770 / 409.819 |
| MSPT mean, ms | 213.535 |
| TPS estimate (1000/p50) | 5.5 |
| heap used / max, MB | 2477.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |

