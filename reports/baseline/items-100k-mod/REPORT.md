# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54568 -> 54568) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 50.343 / 54.110 / 69.143 / 101.658 |
| MSPT mean, ms | 51.174 |
| TPS estimate (1000/p50) | 19.9 |
| heap used / max, MB | 1023.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 7.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 5.98% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.00% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 6 | 4.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 4.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 3.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 9 | 3.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 2.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 11 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1430` |
| 12 | 2.46% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 13 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 15 | 1.29% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |

