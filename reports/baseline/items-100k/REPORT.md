# Baseline run: `items-100k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63059 -> 63059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.813 / 126.059 / 191.561 / 264.993 |
| MSPT mean, ms | 84.356 |
| TPS estimate (1000/p50) | 14.1 |
| heap used / max, MB | 2797.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

