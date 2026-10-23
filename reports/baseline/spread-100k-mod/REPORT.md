# Baseline run: `spread-100k-mod`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97419 -> 97419) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 222.852 / 279.918 / 388.157 / 447.142 |
| MSPT mean, ms | 226.228 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 2039.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 11 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |

