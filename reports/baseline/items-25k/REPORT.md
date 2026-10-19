# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 20005 -> 20005) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.151 / 25.160 / 36.209 / 58.411 |
| MSPT mean, ms | 23.027 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 905.3 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 4.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.09% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 1.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 8 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 15 | 1.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |

