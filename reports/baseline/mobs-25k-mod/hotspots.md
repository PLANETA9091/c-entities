# JVM hotspots (JFR ExecutionSample)

Samples: 23348

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.61% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4423` |
| 5 | 1.38% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4394` |
| 6 | 1.37% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 7 | 1.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.03% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 10 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.98% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |
| 12 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1405` |
| 14 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 19 | 0.85% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.76% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4277` |
| 24 | 0.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 26 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.60% | `java.lang.String.hashCode() line: 2435` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 31 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 32 | 0.58% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3781` |
| 34 | 0.57% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 35 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 36 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 37 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 38 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1424` |
| 39 | 0.52% | `java.lang.String.hashCode() line: 2430` |
| 40 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1408` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 45 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 47 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 49 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3815` |
| 51 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3477` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 57 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 59 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1058` |
| 4 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.63% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 7 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.29% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 13 | 1.03% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3965` |
| 15 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |
| 17 | 0.89% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.89% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 19 | 0.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1044` |
| 21 | 0.78% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 3909` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2647` |
| 25 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.69% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 29 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3995` |

GC pauses: 0, total 0 ms

