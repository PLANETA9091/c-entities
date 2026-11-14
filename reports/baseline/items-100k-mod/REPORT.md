# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62886 -> 62885) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.541 / 114.263 / 248.803 / 329.596 |
| MSPT mean, ms | 80.346 |
| TPS estimate (1000/p50) | 14.8 |
| heap used / max, MB | 2234.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 2 | 6.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 3 | 2.75% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 9 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12739` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.getFluidHeight(TagKey) line: 176` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 14 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.92% | `net.minecraft.core.Vec3i.getY() line: 67` |

