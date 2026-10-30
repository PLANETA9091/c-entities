# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62908 -> 62908) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.078 / 78.019 / 144.363 / 185.746 |
| MSPT mean, ms | 68.992 |
| TPS estimate (1000/p50) | 16.1 |
| heap used / max, MB | 1724.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.13% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 12 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.41% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |

