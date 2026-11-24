# Baseline run: `spread-100k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88781 -> 88649) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 39.088 / 60.730 / 187.227 / 249.423 |
| MSPT mean, ms | 47.199 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2121.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.65% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 2 | 5.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10799` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9548` |
| 5 | 3.10% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.19% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16310` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15210` |
| 9 | 1.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6030` |
| 12 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6776` |
| 14 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

