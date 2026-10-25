# JVM hotspots (JFR ExecutionSample)

Samples: 16444

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4065` |
| 3 | 2.26% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.87% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4094` |
| 5 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 8 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 13 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 19 | 0.85% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 3948` |
| 22 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3452` |
| 24 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1422` |
| 26 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3138` |
| 28 | 0.72% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 31 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 32 | 0.64% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1493` |
| 33 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.61% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 39 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.52% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 44 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3148` |
| 46 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3441` |
| 49 | 0.42% | `java.util.ArrayDeque.size() line: 647` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3486` |
| 51 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3548` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.38% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 55 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 60 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1056` |
| 3 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 7 | 2.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.87% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 9 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3636` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2355` |
| 13 | 1.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 15 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.00% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 19 | 0.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 20 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3666` |
| 21 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.87% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 23 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 24 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.85% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 26 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 28 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.75% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 3580` |

GC pauses: 0, total 0 ms

