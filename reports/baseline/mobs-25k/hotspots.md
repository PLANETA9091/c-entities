# JVM hotspots (JFR ExecutionSample)

Samples: 39201

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.19% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.98% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 0.91% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.83% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.74% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.69% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.66% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 24 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 26 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 33 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.44% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 40 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 45 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.30% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 49 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.25% | `java.util.Optional.isPresent() line: 154` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 58 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 59 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.23% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.33% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.98% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.85% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.83% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 12 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 0.70% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 22 | 0.52% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 25 | 0.50% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 28 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

