# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62965 -> 62965) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.329 / 81.319 / 146.652 / 209.985 |
| MSPT mean, ms | 70.406 |
| TPS estimate (1000/p50) | 15.5 |
| heap used / max, MB | 1116.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.20% | `java.util.ArrayList$Itr.<init>(ArrayList) line: 1042` |
| 2 | 5.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.86% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.20% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.96% | `net.minecraft.core.Vec3i.getY() line: 67` |

