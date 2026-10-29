# JVM hotspots (JFR ExecutionSample)

Samples: 18267

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.75% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 4 | 1.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.50% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8205` |
| 8 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 10 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 12 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 14 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.83% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.74% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 20 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.60% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8272` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4955` |
| 27 | 0.59% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 28 | 0.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 33 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.46% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 35 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 41 | 0.41% | `java.lang.ref.Reference.get() line: 361` |
| 42 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6117` |
| 48 | 0.36% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 49 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8150` |
| 52 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3360` |
| 54 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 56 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.31% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.30% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 1.75% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 4 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8077` |
| 5 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 14 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 17 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 0.80% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 22 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4905` |
| 24 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.69% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 27 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |

GC pauses: 0, total 0 ms

