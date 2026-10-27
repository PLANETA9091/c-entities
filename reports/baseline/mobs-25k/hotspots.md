# JVM hotspots (JFR ExecutionSample)

Samples: 39019

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.90% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.74% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 17 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.63% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 26 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 31 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 49 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.25% | `java.util.Optional.get() line: 142` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.22% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.30% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.90% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 0.79% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.78% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 19 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 25 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |

GC pauses: 0, total 0 ms

