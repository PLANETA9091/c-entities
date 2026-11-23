# JVM hotspots (JFR ExecutionSample)

Samples: 73075

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 51.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 1.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 0.62% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 10 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 0.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 20 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.26% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 25 | 0.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.22% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 31 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.21% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.20% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.19% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.18% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 39 | 0.18% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 42 | 0.18% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 43 | 0.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.18% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.17% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 46 | 0.17% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.17% | `net.minecraft.core.BlockPos.betweenCornersInDirection(int, int, int, int, int, int, Vec3) line: 596` |
| 48 | 0.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 49 | 0.16% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 50 | 0.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 51 | 0.15% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.15% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 53 | 0.14% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 54 | 0.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 55 | 0.14% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 56 | 0.13% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 57 | 0.13% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 58 | 0.13% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 59 | 0.13% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 60 | 0.13% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 51.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.18% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.64% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 7 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.46% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 9 | 0.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.40% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 15 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 16 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 17 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 21 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.27% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 25 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 30 | 0.25% | `net.minecraft.core.BlockPos.betweenCornersInDirection(AABB, Vec3) line: 562` |

GC pauses: 0, total 0 ms

