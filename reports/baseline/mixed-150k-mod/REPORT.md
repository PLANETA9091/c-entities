# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 94835 -> 94835) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 117.978 / 153.742 / 219.388 / 322.951 |
| MSPT mean, ms | 126.508 |
| TPS estimate (1000/p50) | 8.5 |
| heap used / max, MB | 1438.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 2 | 5.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 4 | 4.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 5 | 3.81% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9532` |
| 6 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 9 | 2.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 10 | 2.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 11 | 2.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

