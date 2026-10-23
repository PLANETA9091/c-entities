# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54651 -> 54651) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 102.156 / 108.412 / 120.376 / 162.360 |
| MSPT mean, ms | 103.053 |
| TPS estimate (1000/p50) | 9.8 |
| heap used / max, MB | 1036.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 8.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 6.69% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 5.23% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 7 | 4.60% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 8 | 3.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.93% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 2.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 13 | 1.69% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1421` |

