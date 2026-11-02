# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62930 -> 62930) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 74.546 / 96.091 / 178.197 / 246.996 |
| MSPT mean, ms | 81.973 |
| TPS estimate (1000/p50) | 13.4 |
| heap used / max, MB | 1165.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.67% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

