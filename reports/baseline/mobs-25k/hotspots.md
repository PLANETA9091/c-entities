# JVM hotspots (JFR ExecutionSample)

Samples: 40059

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.08% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.84% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.66% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 21 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 30 | 0.46% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 31 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 36 | 0.42% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 40 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 43 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 44 | 0.34% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 45 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.31% | `java.util.ArrayDeque.size() line: 647` |
| 51 | 0.27% | `java.lang.String.hashCode() line: 2430` |
| 52 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.24% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.21% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.08% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 0.83% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.60% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 16 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.58% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.52% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.51% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 30 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |

GC pauses: 0, total 0 ms

