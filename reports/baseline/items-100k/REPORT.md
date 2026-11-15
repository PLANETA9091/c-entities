# Baseline run: `items-100k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62945 -> 62945) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.829 / 114.473 / 178.884 / 255.544 |
| MSPT mean, ms | 80.973 |
| TPS estimate (1000/p50) | 14.5 |
| heap used / max, MB | 1095.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.92% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 6 | 2.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

