# JVM hotspots (JFR ExecutionSample)

Samples: 16509

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.82% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 5 | 1.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 6 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.38% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.10% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 0.98% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 13 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 16 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 19 | 0.68% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.62% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.61% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 30 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 34 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 35 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 38 | 0.41% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4474` |
| 39 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 40 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 41 | 0.39% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 42 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 48 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.33% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |
| 52 | 0.33% | `java.lang.ThreadLocal.get() line: 171` |
| 53 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 55 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2562` |
| 58 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5124` |
| 60 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 4 | 1.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 5 | 1.59% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5872` |
| 6 | 1.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.41% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5870` |
| 9 | 1.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 10 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 12 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 14 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.10% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.02% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.94% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 21 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 5978` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.67% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 6011` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.65% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

