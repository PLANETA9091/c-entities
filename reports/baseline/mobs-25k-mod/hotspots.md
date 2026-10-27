# JVM hotspots (JFR ExecutionSample)

Samples: 15359

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.49% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 6 | 1.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 11 | 1.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 1.12% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 17 | 1.11% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 18 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 20 | 0.89% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 23 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 25 | 0.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.70% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6588` |
| 27 | 0.69% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.58% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 34 | 0.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5631` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7762` |
| 37 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7640` |
| 39 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 43 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6784` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 46 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2884` |
| 48 | 0.42% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6317` |
| 50 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 51 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 52 | 0.40% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 53 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 55 | 0.38% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4888` |
| 56 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.36% | `dev.planeta9091.centities.CeHooks.aiTail(Mob) line: 7075` |
| 59 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 60 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 4 | 2.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 5 | 2.49% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 6 | 1.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 11 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7567` |
| 16 | 1.11% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 17 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.05% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 20 | 1.00% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7550` |
| 22 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.76% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 25 | 0.72% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6742` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4419` |
| 30 | 0.67% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |

GC pauses: 0, total 0 ms

