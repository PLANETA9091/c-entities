# Baseline run: `items-100k-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62960 -> 62959) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.115 / 29.161 / 115.379 / 233.901 |
| MSPT mean, ms | 22.742 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1388.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9497` |
| 2 | 7.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 3 | 5.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 6 | 2.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15432` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 9 | 1.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15206` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

