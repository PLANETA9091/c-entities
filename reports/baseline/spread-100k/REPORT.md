# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97376 -> 97371) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 351.747 / 416.770 / 493.866 / 534.799 |
| MSPT mean, ms | 360.755 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 2758.3 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 2.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 12 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 14 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

