# Baseline run: `mixed-150k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110428 -> 110428) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 536.277 / 668.941 / 961.302 / 1042.313 |
| MSPT mean, ms | 561.409 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2151.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 4.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 3.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.69% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 10 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

