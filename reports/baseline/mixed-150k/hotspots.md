# JVM hotspots (JFR ExecutionSample)

Samples: 104668

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 17 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.60% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.setNextRelease(ConcurrentChainedLong2ReferenceHashTable$TableEntry) line: 1850` |
| 21 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.54% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 25 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 33 | 0.39% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 34 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 37 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 38 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 39 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 42 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 44 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.27% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 48 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 51 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 55 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 58 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 60 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 0.79% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 16 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.remove(long, Object) line: 883` |
| 17 | 0.57% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.52% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.47% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.42% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 27 | 0.41% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.39% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 30 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

