# JVM hotspots (JFR ExecutionSample)

Samples: 42207

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.44% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 8 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 20 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 22 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 35 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 38 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 39 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 40 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.28% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 42 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 44 | 0.26% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 45 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 46 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 47 | 0.24% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.24% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 52 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 53 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.22% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 56 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 58 | 0.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.20% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 60 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.41% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 8 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.73% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 14 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 16 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 17 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 19 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 20 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 0.48% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 25 | 0.45% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.45% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.45% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.44% | `java.util.HashMap.get(Object) line: 565` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

