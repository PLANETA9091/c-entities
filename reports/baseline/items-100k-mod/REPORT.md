# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62899 -> 62899) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.743 / 68.262 / 106.997 / 146.024 |
| MSPT mean, ms | 60.757 |
| TPS estimate (1000/p50) | 17.3 |
| heap used / max, MB | 2468.7 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 2.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 3 | 2.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 6 | 2.02% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 7 | 1.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 14 | 1.34% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 15 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |

