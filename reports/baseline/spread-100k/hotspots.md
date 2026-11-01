# JVM hotspots (JFR ExecutionSample)

Samples: 85006

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 0.91% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 18 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 19 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 35 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.36% | `java.util.HashMap.getNode(Object) line: 586` |
| 38 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 40 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 41 | 0.34% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 42 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 43 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 47 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 49 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 55 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 56 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 60 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.26% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.91% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 16 | 0.82% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.79% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 24 | 0.58% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.49% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.44% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

