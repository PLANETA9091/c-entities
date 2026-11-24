# Baseline run: `spread-100k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88831 -> 88709) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 44.130 / 64.019 / 194.846 / 376.439 |
| MSPT mean, ms | 52.935 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2081.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9547` |
| 3 | 5.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 4 | 5.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 7 | 1.68% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 9 | 1.56% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6025` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.93% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 0.85% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |

