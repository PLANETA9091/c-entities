# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62988 -> 62987) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 78.234 / 136.784 / 201.049 / 261.627 |
| MSPT mean, ms | 91.800 |
| TPS estimate (1000/p50) | 12.8 |
| heap used / max, MB | 2221.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 4 | 3.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.26% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

