# JVM hotspots (JFR ExecutionSample)

Samples: 47891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.61% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.43% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 26 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.38% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 30 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 33 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 37 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 38 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 39 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 43 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.30% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 46 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 49 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 52 | 0.27% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 53 | 0.27% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 54 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 55 | 0.26% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 57 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 60 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 6 | 1.18% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.70% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 10 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 15 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 0.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 21 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.46% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 26 | 0.46% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 27 | 0.44% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 28 | 0.42% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 29 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

