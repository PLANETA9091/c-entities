# JVM hotspots (JFR ExecutionSample)

Samples: 18363

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.60% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 5 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.28% | `java.lang.ref.Reference.get() line: 361` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 9 | 1.71% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 14 | 1.33% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 15 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.02% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 20 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 21 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.83% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5850` |
| 26 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 31 | 0.64% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 32 | 0.64% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 33 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5299` |
| 34 | 0.63% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 37 | 0.58% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 38 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 39 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 41 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3094` |
| 42 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 43 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 45 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 51 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 52 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 5158` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.32% | `java.util.stream.AbstractPipeline.copyIntoWithCancel(Sink, Spliterator) line: 589` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 2 | 4.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 2.60% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 7 | 2.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.17% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5011` |
| 18 | 1.12% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 19 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.89% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 22 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.87% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5846` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5298` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4638` |
| 30 | 0.71% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |

GC pauses: 0, total 0 ms

