# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62904 -> 62904) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.563 / 44.922 / 107.137 / 173.536 |
| MSPT mean, ms | 27.243 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1259.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5494` |
| 2 | 7.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4056` |
| 3 | 4.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3929` |
| 4 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6272` |
| 7 | 2.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5499` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4009` |
| 9 | 2.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 2.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2724` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 13 | 1.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5570` |
| 15 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

