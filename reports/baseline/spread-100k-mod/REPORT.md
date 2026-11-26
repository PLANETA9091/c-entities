# Baseline run: `spread-100k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88718 -> 88589) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 42.734 / 69.036 / 161.949 / 473.570 |
| MSPT mean, ms | 50.720 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2144.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 2 | 5.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9580` |
| 4 | 3.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 2.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16577` |
| 9 | 1.87% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.55% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.35% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.32% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.discard(EntityRemoveEvent$Cause) line: 685` |

