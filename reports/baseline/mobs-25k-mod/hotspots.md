# JVM hotspots (JFR ExecutionSample)

Samples: 21072

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7837` |
| 6 | 1.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7770` |
| 7 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 9 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1870` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 15 | 1.05% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1851` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4689` |
| 19 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6663` |
| 21 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 24 | 0.72% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 25 | 0.68% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 26 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.58% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4544` |
| 30 | 0.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.51% | `java.lang.ref.Reference.get() line: 361` |
| 34 | 0.51% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7715` |
| 39 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6859` |
| 42 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 44 | 0.38% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 45 | 0.38% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 46 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2959` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1854` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3022` |
| 54 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.31% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.30% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 60 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1393` |
| 3 | 1.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1855` |
| 6 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7642` |
| 7 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.32% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 9 | 1.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 12 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.06% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 17 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7617` |
| 19 | 0.85% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 21 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 25 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map) line: 134` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6829` |
| 27 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 29 | 0.72% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 30 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

