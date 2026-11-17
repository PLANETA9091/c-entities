# JVM hotspots (JFR ExecutionSample)

Samples: 55493

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.54% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 30 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 32 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.34% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 34 | 0.33% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 35 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 36 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 39 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 40 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 44 | 0.28% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 45 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 49 | 0.28% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 50 | 0.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 52 | 0.24% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 56 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.22% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.21% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 0.90% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 12 | 0.58% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 20 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 22 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 23 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 27 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.36% | `java.util.ArrayList.toArray() line: 370` |

GC pauses: 0, total 0 ms

