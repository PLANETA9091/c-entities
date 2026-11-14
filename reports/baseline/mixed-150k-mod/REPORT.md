# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113868 -> 113836) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 90.921 / 172.141 / 328.418 / 421.816 |
| MSPT mean, ms | 111.767 |
| TPS estimate (1000/p50) | 11.0 |
| heap used / max, MB | 2099.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 2 | 9.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 3 | 7.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 4 | 4.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.59% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 11 | 1.77% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |

