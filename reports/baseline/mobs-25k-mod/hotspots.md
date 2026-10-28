# JVM hotspots (JFR ExecutionSample)

Samples: 20401

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 8 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 9 | 1.19% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 10 | 1.03% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.98% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 17 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 22 | 0.66% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 23 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.64% | `java.lang.ref.Reference.get() line: 361` |
| 25 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 32 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.42% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 43 | 0.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 45 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 48 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 52 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.34% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 54 | 0.33% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 55 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 57 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 59 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.39% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 8 | 1.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 11 | 1.19% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 12 | 1.07% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 1.07% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.06% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 15 | 0.98% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6973` |
| 24 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6792` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4638` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

