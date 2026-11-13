# JVM hotspots (JFR ExecutionSample)

Samples: 56708

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.89% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.88% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 9 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 12 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 27 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 28 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 37 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.27% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 44 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.25% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 48 | 0.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.25% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.24% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 51 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.21% | `java.util.ArrayDeque.size() line: 647` |
| 54 | 0.21% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 55 | 0.20% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 60 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.57% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.90% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 7 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 9 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.60% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 14 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.43% | `java.util.HashMap.get(Object) line: 565` |
| 21 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 25 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.37% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 27 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 29 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos) line: 863` |
| 30 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |

GC pauses: 0, total 0 ms

