# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19978 -> 19978) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.318 / 20.982 / 32.713 / 46.736 |
| MSPT mean, ms | 19.398 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1186.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 2 | 3.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 2.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 1.54% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 6 | 1.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.41% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 10 | 1.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 1.25% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 12 | 1.19% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 15 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

