# Baseline run: `items-100k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62952 -> 62952) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.087 / 79.720 / 148.388 / 201.148 |
| MSPT mean, ms | 71.487 |
| TPS estimate (1000/p50) | 15.4 |
| heap used / max, MB | 1638.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.94% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

