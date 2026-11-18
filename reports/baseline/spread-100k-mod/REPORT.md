# Baseline run: `spread-100k-mod`

Server: `26.3-255-c22378f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88758 -> 88626) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 40.824 / 64.013 / 174.598 / 225.590 |
| MSPT mean, ms | 48.531 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2420.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 3 | 3.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 4 | 3.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8777` |
| 5 | 2.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.99% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15013` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15031` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1030` |
| 12 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5711` |
| 15 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |

