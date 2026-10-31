# JVM hotspots (JFR ExecutionSample)

Samples: 53032

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.67% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 13 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 17 | 0.57% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 18 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.51% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 30 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 31 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 33 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 38 | 0.31% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 39 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.27% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.26% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 46 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.23% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 50 | 0.23% | `java.lang.Double.isFinite(double) line: 1023` |
| 51 | 0.22% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 53 | 0.21% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 54 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 55 | 0.21% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.find(Object) line: 238` |
| 57 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.20% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |
| 59 | 0.20% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.93% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.69% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.67% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 11 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 15 | 0.57% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 16 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.42% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.39% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 30 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms

