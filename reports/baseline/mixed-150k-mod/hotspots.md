# JVM hotspots (JFR ExecutionSample)

Samples: 27240

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7369` |
| 2 | 6.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 5 | 3.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.60% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 12 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.19% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 16 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.77% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11857` |
| 26 | 0.68% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.58% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 34 | 0.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 37 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11853` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 41 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12451` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 12550` |
| 45 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 46 | 0.41% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.40% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 387` |
| 49 | 0.40% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 50 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 51 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11899` |
| 53 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 56 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 58 | 0.32% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 59 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 60 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 7.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 3.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 7 | 2.38% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 1.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 10 | 1.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 11 | 1.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 13 | 1.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8403` |
| 15 | 1.38% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.24% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7390` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.85% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 23 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 25 | 0.76% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 26 | 0.75% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 27 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.66% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10269` |

GC pauses: 0, total 0 ms

