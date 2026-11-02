# JVM hotspots (JFR ExecutionSample)

Samples: 109879

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.51% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.26% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.55% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 32 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.31% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 38 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 40 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 41 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 42 | 0.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 44 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.23% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 51 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.21% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 58 | 0.20% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.20% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 60 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.10% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.94% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.65% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.46% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.38% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.38% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.36% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 30 | 0.34% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |

GC pauses: 0, total 0 ms

