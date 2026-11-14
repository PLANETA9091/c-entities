# JVM hotspots (JFR ExecutionSample)

Samples: 24318

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7826` |
| 2 | 5.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 5 | 3.99% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 11 | 1.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.13% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.10% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 18 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 20 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.86% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.86% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.62% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 31 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.59% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 34 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12311` |
| 37 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.44% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 41 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.39% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 46 | 0.39% | `java.lang.ThreadLocal.get() line: 171` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12917` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7064` |
| 49 | 0.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 50 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7816` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12357` |
| 54 | 0.32% | `java.util.ArrayDeque.size() line: 647` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.28% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 59 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12315` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 8.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 11 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12544` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.13% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 1.01% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.00% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2921` |
| 21 | 0.97% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 22 | 0.92% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 23 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.86% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8860` |
| 26 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 27 | 0.81% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 29 | 0.66% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |

GC pauses: 0, total 0 ms

