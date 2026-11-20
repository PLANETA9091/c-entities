# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62998 -> 62998) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.955 / 28.042 / 96.451 / 165.400 |
| MSPT mean, ms | 20.778 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1270.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 5.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 5 | 3.85% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14688` |
| 8 | 1.49% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 9 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6512` |
| 11 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1017` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6183` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

