# Baseline run: `items-100k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62930 -> 62930) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.049 / 22.802 / 95.480 / 229.196 |
| MSPT mean, ms | 18.151 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1272.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.71% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 3 | 4.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 4 | 3.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9416` |
| 5 | 3.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15351` |
| 9 | 1.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15125` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6705` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9162` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16270` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

