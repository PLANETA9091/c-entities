# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62942 -> 62943) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 35.249 / 57.940 / 186.802 / 219.398 |
| MSPT mean, ms | 44.171 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1705.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 2 | 7.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 3 | 5.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.05% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7700` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13390` |
| 9 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12910` |
| 11 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13812` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |

