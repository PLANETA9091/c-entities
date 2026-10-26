# JVM hotspots (JFR ExecutionSample)

Samples: 37725

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.86% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 0.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.63% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 20 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.50% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 30 | 0.44% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.44% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 32 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.40% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 36 | 0.39% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 43 | 0.32% | `java.util.Optional.isPresent() line: 154` |
| 44 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 45 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.28% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 49 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 51 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 55 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.22% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 57 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.21% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.38% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.86% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 0.86% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.63% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.55% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 21 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 22 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 23 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.49% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 29 | 0.46% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.46% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

