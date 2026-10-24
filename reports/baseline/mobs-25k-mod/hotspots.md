# JVM hotspots (JFR ExecutionSample)

Samples: 19867

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.17% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 3 | 1.89% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 4 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.46% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 6 | 1.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.12% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 10 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 12 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1384` |
| 17 | 0.88% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 18 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 20 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 21 | 0.75% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 31 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 32 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 33 | 0.61% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.61% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.57% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 37 | 0.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.37% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 50 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.35% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 56 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.34% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 59 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 4 | 1.89% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 5 | 1.60% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 6 | 1.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.49% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 8 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1388` |
| 11 | 1.29% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2218` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 16 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 19 | 0.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.87% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 21 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.78% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 26 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3374` |
| 28 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

