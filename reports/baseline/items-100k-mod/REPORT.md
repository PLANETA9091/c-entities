# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 53442 -> 53442) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 46.107 / 96.668 / 103.011 / 157.672 |
| MSPT mean, ms | 65.218 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1564.8 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 8.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 8.74% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 7.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.23% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 7 | 3.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 8 | 1.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 10 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.00% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 15 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 219` |

