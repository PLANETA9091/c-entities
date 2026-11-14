# Baseline run: `items-100k`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63008 -> 63008) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 69.152 / 116.852 / 173.338 / 201.010 |
| MSPT mean, ms | 81.482 |
| TPS estimate (1000/p50) | 14.5 |
| heap used / max, MB | 1369.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.67% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.65% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

