# Baseline run: `items-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62973 -> 62973) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 80.257 / 145.824 / 224.650 / 322.272 |
| MSPT mean, ms | 95.799 |
| TPS estimate (1000/p50) | 12.5 |
| heap used / max, MB | 2044.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 4 | 3.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.24% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

