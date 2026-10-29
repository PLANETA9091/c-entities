# JVM hotspots (JFR ExecutionSample)

Samples: 18546

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.63% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 9 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 12 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 16 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.92% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 19 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.68% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.62% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 27 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 29 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 32 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8044` |
| 34 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.44% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 38 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 42 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 44 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5913` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 51 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 55 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 57 | 0.32% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3219` |
| 59 | 0.32% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 3 | 2.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 8 | 1.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.21% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7849` |
| 15 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.95% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.94% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 20 | 0.92% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 21 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4701` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.74% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.67% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |

GC pauses: 0, total 0 ms

