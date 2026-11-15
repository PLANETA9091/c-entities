# Baseline run: `mobs-25k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24158 -> 24158) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 526.787 / 594.737 / 690.368 / 953.447 |
| MSPT mean, ms | 535.452 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 989.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.03% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 0.95% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 10 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.58% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 15 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

