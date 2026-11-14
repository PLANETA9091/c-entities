# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109405 -> 108927) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 111.008 / 182.486 / 349.019 / 515.771 |
| MSPT mean, ms | 131.846 |
| TPS estimate (1000/p50) | 9.0 |
| heap used / max, MB | 2731.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 2 | 5.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 4 | 3.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 11 | 1.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12924` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

