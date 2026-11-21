# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62987 -> 62987) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 75.128 / 121.256 / 181.296 / 803.678 |
| MSPT mean, ms | 87.636 |
| TPS estimate (1000/p50) | 13.3 |
| heap used / max, MB | 1769.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 3.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 3.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 11 | 1.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |

