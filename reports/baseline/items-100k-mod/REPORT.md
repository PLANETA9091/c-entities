# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 20604 -> 5643) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.137 / 30.743 / 43.056 / 81.494 |
| MSPT mean, ms | 16.932 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1686.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 2 | 3.59% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 3 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2008` |
| 5 | 2.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 2.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 8 | 2.16% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1415` |
| 11 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |

