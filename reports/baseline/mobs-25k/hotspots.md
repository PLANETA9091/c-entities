# JVM hotspots (JFR ExecutionSample)

Samples: 51369

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.87% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.87% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 13 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.44% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 26 | 0.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.35% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 32 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.33% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 46 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.28% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 48 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.27% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.26% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 52 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 53 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 56 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 60 | 0.21% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.87% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.77% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.48% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 20 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 22 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.45% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 25 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 27 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.42% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.41% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |

GC pauses: 0, total 0 ms

