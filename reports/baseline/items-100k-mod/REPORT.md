# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54738 -> 54738) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 41.728 / 61.149 / 132.589 / 213.200 |
| MSPT mean, ms | 48.058 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1024.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 11.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 3 | 7.70% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 4 | 5.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2750` |
| 6 | 3.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2647` |
| 7 | 3.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2107` |
| 9 | 2.24% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3138` |
| 11 | 1.91% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2010` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2061` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3165` |

