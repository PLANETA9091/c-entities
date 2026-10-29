# JVM hotspots (JFR ExecutionSample)

Samples: 51688

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.63% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 15 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 18 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.48% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.37% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 29 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 30 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 31 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 35 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 38 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 41 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.28% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 43 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 49 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 51 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.23% | `java.util.Optional.isPresent() line: 154` |
| 55 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.22% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.21% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.21% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 0.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.63% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 12 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 19 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.44% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 26 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 28 | 0.39% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 29 | 0.39% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 30 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

