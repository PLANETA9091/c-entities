# JVM hotspots (JFR ExecutionSample)

Samples: 53961

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.03% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.98% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 14 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 24 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 27 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 29 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 30 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 31 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 35 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 36 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.31% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 40 | 0.31% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 41 | 0.31% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 42 | 0.30% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 43 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 45 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 46 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 47 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 283` |
| 48 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 51 | 0.22% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 53 | 0.22% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 54 | 0.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 55 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.21% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.21% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |
| 58 | 0.21% | `java.lang.ThreadLocal.get() line: 171` |
| 59 | 0.21% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 60 | 0.20% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.04% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.94% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.83% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 9 | 0.80% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 10 | 0.72% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.71% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 12 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 18 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.44% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 22 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.41% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 25 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos) line: 863` |
| 29 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 30 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

