# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62964 -> 62964) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.628 / 51.975 / 115.001 / 189.564 |
| MSPT mean, ms | 37.019 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1650.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 2 | 5.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 3 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 6 | 2.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13209` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |

