# JVM hotspots (JFR ExecutionSample)

Samples: 29714

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7425` |
| 2 | 4.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12048` |
| 3 | 3.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.26% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 14 | 1.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 16 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.88% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 440` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11910` |
| 25 | 0.67% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 26 | 0.66% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.58% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 31 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.51% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 37 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 39 | 0.47% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 40 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 42 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 47 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.42% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 49 | 0.41% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 50 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 52 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 54 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11914` |
| 57 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8448` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 6.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2713` |
| 6 | 2.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 1.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.84% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 11 | 1.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12143` |
| 13 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8459` |
| 17 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7446` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 21 | 0.87% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.85% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 24 | 0.83% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.81% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 454` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.66% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10325` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.getSharedFlag$(Entity, int) line: 8858` |

GC pauses: 0, total 0 ms

