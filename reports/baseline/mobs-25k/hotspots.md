# JVM hotspots (JFR ExecutionSample)

Samples: 49972

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 0.78% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.55% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 28 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 30 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 31 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 37 | 0.33% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 39 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 44 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 47 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.27% | `java.util.ArrayDeque.size() line: 647` |
| 50 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.26% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 52 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.24% | `java.lang.String.hashCode() line: 2430` |
| 57 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 59 | 0.22% | `java.lang.ThreadLocal.get() line: 171` |
| 60 | 0.22% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.85% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 0.58% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.45% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.40% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 30 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

