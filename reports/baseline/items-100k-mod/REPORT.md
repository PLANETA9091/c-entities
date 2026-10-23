# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54612 -> 54611) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.039 / 66.826 / 87.954 / 137.601 |
| MSPT mean, ms | 64.376 |
| TPS estimate (1000/p50) | 15.9 |
| heap used / max, MB | 1530.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 10.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 6.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 4.25% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 3.09% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 8 | 2.61% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 9 | 2.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 2.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 2.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 12 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1430` |
| 14 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |

