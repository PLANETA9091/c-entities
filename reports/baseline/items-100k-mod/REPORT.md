# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54503 -> 54499) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.604 / 88.897 / 118.112 / 165.355 |
| MSPT mean, ms | 62.647 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1890.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 7.39% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 6.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 5.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.90% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 3.66% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 8 | 3.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 10 | 2.56% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 663` |
| 11 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 15 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |

