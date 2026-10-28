# Baseline run: `spread-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97433 -> 97433) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 455.933 / 526.587 / 756.263 / 850.741 |
| MSPT mean, ms | 467.355 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 1998.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |

