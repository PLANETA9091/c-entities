# Baseline run: `spread-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88833 -> 88709) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.295 / 96.070 / 206.540 / 241.049 |
| MSPT mean, ms | 68.605 |
| TPS estimate (1000/p50) | 17.2 |
| heap used / max, MB | 2680.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7819` |
| 2 | 7.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 3 | 6.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 4 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.79% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5291` |
| 9 | 2.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 2.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5031` |
| 12 | 1.71% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 13 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

