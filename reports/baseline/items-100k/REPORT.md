# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63008 -> 63008) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.913 / 116.771 / 172.564 / 230.678 |
| MSPT mean, ms | 81.823 |
| TPS estimate (1000/p50) | 14.5 |
| heap used / max, MB | 1397.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.39% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 5 | 2.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.37% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

