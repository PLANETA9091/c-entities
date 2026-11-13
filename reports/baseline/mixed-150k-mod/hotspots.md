# JVM hotspots (JFR ExecutionSample)

Samples: 28235

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 2 | 6.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 4.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 5 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.08% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.80% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.75% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.70% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11844` |
| 28 | 0.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 30 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 33 | 0.60% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12438` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11840` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 38 | 0.48% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 39 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 40 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11886` |
| 43 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 44 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 48 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 49 | 0.39% | `java.lang.ThreadLocal.get() line: 171` |
| 50 | 0.38% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 387` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 52 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 56 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 57 | 0.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 58 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 7.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 6 | 2.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.23% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.10% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 9 | 2.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 11 | 1.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8390` |
| 13 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 14 | 1.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 1.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 16 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.12% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 1.08% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.79% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 25 | 0.78% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.75% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10256` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |

GC pauses: 0, total 0 ms

