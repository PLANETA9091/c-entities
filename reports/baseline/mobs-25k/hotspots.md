# JVM hotspots (JFR ExecutionSample)

Samples: 51285

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.66% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 23 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 27 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.32% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 39 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 42 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 44 | 0.29% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 45 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 46 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 50 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 52 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.24% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 55 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 56 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.23% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 58 | 0.21% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.21% | `java.util.ArrayDeque.size() line: 647` |
| 60 | 0.20% | `java.lang.String.hashCode() line: 2430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.94% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 7 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 9 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.49% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 21 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 22 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 24 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.43% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

GC pauses: 0, total 0 ms

