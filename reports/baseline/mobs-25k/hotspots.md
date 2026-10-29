# JVM hotspots (JFR ExecutionSample)

Samples: 48420

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 10 | 0.83% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.76% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 32 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 38 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.34% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 41 | 0.34% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 42 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 44 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 47 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.25% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.23% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.22% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 59 | 0.21% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 60 | 0.21% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 7.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 0.83% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.76% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.57% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 15 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.52% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 0.50% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.49% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 20 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 22 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 23 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 26 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.43% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 30 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

