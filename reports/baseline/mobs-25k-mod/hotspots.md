# JVM hotspots (JFR ExecutionSample)

Samples: 25359

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 4 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 10 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 12 | 1.01% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 17 | 0.83% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 18 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 20 | 0.77% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 21 | 0.77% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 23 | 0.73% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.72% | `java.lang.ref.Reference.get() line: 361` |
| 26 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.58% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 30 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 33 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 34 | 0.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.isNullNibbleVisible() line: 280` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 40 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1847` |
| 43 | 0.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 44 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 47 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 52 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 54 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 56 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 58 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 59 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1973` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 3.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 4 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 9 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 1.01% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 15 | 0.83% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.77% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.76% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4638` |
| 22 | 0.74% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.isInWall() line: 3145` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.60% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int, int, int) line: 402` |
| 30 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

