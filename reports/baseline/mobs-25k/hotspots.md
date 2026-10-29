# JVM hotspots (JFR ExecutionSample)

Samples: 50035

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.97% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.78% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 26 | 0.43% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 30 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 31 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.30% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 39 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 42 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.27% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 45 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 47 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 51 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.25% | `java.lang.String.hashCode() line: 2430` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 55 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.23% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.22% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.01% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.97% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.71% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.42% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 26 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.39% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |

GC pauses: 0, total 0 ms

