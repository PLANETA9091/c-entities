# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 53930 -> 53930) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.230 / 132.842 / 164.793 / 217.745 |
| MSPT mean, ms | 81.487 |
| TPS estimate (1000/p50) | 15.8 |
| heap used / max, MB | 1123.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.17% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 7 | 2.43% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 9 | 1.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1821` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |

