# JVM hotspots (JFR ExecutionSample)

Samples: 30240

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7369` |
| 2 | 4.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 3 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.40% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 16 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 22 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11853` |
| 25 | 0.77% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.73% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 27 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.61% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 32 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 36 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 38 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 39 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 40 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 41 | 0.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12451` |
| 43 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.46% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 48 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 49 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.36% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11857` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 56 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11899` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 12347` |
| 60 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.55% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 6 | 1.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 13 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 1.09% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8403` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7390` |
| 17 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.94% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 22 | 0.82% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 23 | 0.82% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10269` |
| 27 | 0.66% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3453` |
| 29 | 0.64% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.64% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms

