# Baseline run: `items-100k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63064 -> 63063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.509 / 26.499 / 98.930 / 254.108 |
| MSPT mean, ms | 20.885 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1377.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10799` |
| 2 | 7.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9498` |
| 3 | 5.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 2.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6767` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15459` |
| 9 | 1.73% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15207` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9244` |
| 12 | 1.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |

