# JVM hotspots (JFR ExecutionSample)

Samples: 55056

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.03% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 0.95% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 10 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.58% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 15 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 30 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 34 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.32% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 37 | 0.30% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 38 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 39 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 41 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 47 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 54 | 0.21% | `java.lang.ThreadLocal.get() line: 171` |
| 55 | 0.20% | `it.unimi.dsi.fastutil.objects.AbstractReference2ObjectMap.putAll(Map) line: 330` |
| 56 | 0.20% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.19% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.18% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.18% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.73% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 0.97% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 8 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 0.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.68% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 12 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 16 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 21 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 24 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.38% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.37% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.36% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.36% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |

GC pauses: 0, total 0 ms

