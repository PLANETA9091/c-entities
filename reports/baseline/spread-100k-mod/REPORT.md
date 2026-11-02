# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 77349 -> 77349) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 91.042 / 106.962 / 197.899 / 216.865 |
| MSPT mean, ms | 97.949 |
| TPS estimate (1000/p50) | 11.0 |
| heap used / max, MB | 2345.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 2 | 3.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 4 | 3.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 5 | 3.13% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9532` |
| 6 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

