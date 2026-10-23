# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 95945 -> 95469) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 430.899 / 536.768 / 647.761 / 753.596 |
| MSPT mean, ms | 443.032 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2527.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 3 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 11 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry.getValueVolatile() line: 1797` |
| 12 | 1.01% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 13 | 0.99% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

