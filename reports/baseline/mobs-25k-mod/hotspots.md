# JVM hotspots (JFR ExecutionSample)

Samples: 17767

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 4 | 1.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 8 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 10 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7919` |
| 13 | 1.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.19% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.99% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 17 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.89% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.75% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.73% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.70% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 25 | 0.70% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7986` |
| 28 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.54% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 32 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.51% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5855` |
| 35 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.48% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 37 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.43% | `java.lang.ref.Reference.get() line: 361` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 45 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 46 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7864` |
| 49 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 51 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 55 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.33% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3099` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 2 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 4 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7766` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7791` |
| 12 | 1.19% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 13 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.97% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 15 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 17 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.76% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.73% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4643` |
| 27 | 0.71% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.70% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

