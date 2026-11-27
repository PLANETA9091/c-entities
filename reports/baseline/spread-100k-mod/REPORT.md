# Baseline run: `spread-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88745 -> 88612) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 44.618 / 67.557 / 186.959 / 248.305 |
| MSPT mean, ms | 52.908 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2127.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 2 | 5.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9583` |
| 3 | 4.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.61% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 3.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 9 | 1.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13960` |
| 11 | 1.52% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6034` |
| 15 | 0.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

