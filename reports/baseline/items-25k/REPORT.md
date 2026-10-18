# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19889 -> 19889) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.048 / 22.037 / 30.734 / 42.772 |
| MSPT mean, ms | 19.890 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 773.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 5.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.19% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 7 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.64% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 12 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

