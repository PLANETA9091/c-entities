# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97402 -> 97402) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 186.392 / 339.230 / 512.739 / 675.454 |
| MSPT mean, ms | 216.343 |
| TPS estimate (1000/p50) | 5.4 |
| heap used / max, MB | 1914.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8271` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5166` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 6 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4448` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

