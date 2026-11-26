# Baseline run: `mixed-150k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111833 -> 111344) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.444 / 96.025 / 232.829 / 321.219 |
| MSPT mean, ms | 71.326 |
| TPS estimate (1000/p50) | 17.4 |
| heap used / max, MB | 1903.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 2 | 6.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 4 | 4.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9528` |
| 5 | 4.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 7 | 1.79% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.49% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |

