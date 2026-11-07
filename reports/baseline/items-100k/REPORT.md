# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62983 -> 62983) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.568 / 121.317 / 180.436 / 237.998 |
| MSPT mean, ms | 83.599 |
| TPS estimate (1000/p50) | 14.2 |
| heap used / max, MB | 2258.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.92% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 6 | 2.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 11 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.53% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.33% | `java.util.EnumMap.get(Object) line: 248` |

