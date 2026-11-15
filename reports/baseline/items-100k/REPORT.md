# Baseline run: `items-100k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62920 -> 62920) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 75.266 / 136.036 / 202.899 / 278.650 |
| MSPT mean, ms | 89.961 |
| TPS estimate (1000/p50) | 13.3 |
| heap used / max, MB | 2303.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 2.95% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 5 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.56% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.22% | `net.minecraft.core.Vec3i.getY() line: 67` |

