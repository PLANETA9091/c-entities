# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97386 -> 97379) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 403.884 / 465.305 / 543.383 / 590.082 |
| MSPT mean, ms | 407.437 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 2294.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 2.05% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.40% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

