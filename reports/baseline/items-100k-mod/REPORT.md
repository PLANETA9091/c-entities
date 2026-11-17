# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62951 -> 62952) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.918 / 47.825 / 117.462 / 189.365 |
| MSPT mean, ms | 29.488 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1641.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7612` |
| 4 | 4.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 5 | 2.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8571` |
| 6 | 2.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 7 | 2.17% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 8 | 1.73% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.69% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7614` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5905` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5901` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7608` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7599` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13692` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

