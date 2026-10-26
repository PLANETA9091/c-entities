# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 130.902 / 232.989 / 308.937 / 366.665 |
| MSPT mean, ms | 145.923 |
| TPS estimate (1000/p50) | 7.6 |
| heap used / max, MB | 1807.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6709` |
| 4 | 2.29% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.23% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 12 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |

