# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62875 -> 62875) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.446 / 67.844 / 114.719 / 194.492 |
| MSPT mean, ms | 34.404 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1313.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 2 | 6.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3980` |
| 3 | 6.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 6.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2676` |
| 5 | 5.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 6 | 3.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.98% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2629` |
| 8 | 2.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.77% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2578` |
| 11 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |
| 12 | 1.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getType() line: 633` |

