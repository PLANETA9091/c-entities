# Baseline run: `spread-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97400 -> 97400) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 308.019 / 372.602 / 521.560 / 601.819 |
| MSPT mean, ms | 322.121 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2972.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 1.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

