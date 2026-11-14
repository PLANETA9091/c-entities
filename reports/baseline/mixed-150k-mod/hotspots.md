# JVM hotspots (JFR ExecutionSample)

Samples: 25572

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7769` |
| 2 | 5.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 5 | 4.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 10 | 1.83% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 11 | 1.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.08% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7790` |
| 18 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 20 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.85% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.73% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 28 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 29 | 0.60% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 30 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 31 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 33 | 0.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 34 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12254` |
| 36 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 38 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 40 | 0.43% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 42 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.38% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 48 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 50 | 0.38% | `java.lang.ThreadLocal.get() line: 171` |
| 51 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9356` |
| 53 | 0.35% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 12951` |
| 54 | 0.34% | `java.lang.String.hashCode() line: 2434` |
| 55 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 57 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7759` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12300` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 60 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 9.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.57% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 3.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 10 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8803` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 17 | 1.08% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 18 | 1.02% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 21 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 23 | 0.85% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 25 | 0.82% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.73% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 27 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1429` |

GC pauses: 0, total 0 ms

