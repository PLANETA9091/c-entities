# JVM hotspots (JFR ExecutionSample)

Samples: 52994

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.70% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 19 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 26 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.40% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.push(Entity) line: 2472` |
| 29 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 32 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.32% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.31% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 39 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 41 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 42 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 47 | 0.27% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 48 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.24% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.23% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 53 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 56 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.22% | `java.util.ImmutableCollections$SetN.probe(Object) line: 1258` |
| 59 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 7 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 0.71% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.70% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 12 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 21 | 0.48% | `net.minecraft.world.entity.LivingEntity.push(Entity) line: 2948` |
| 22 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.45% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 25 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.40% | `net.minecraft.world.item.ItemStack.hurtAndBreak(int, ServerLevel, LivingEntity, Consumer, boolean) line: 621` |
| 28 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 29 | 0.39% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

