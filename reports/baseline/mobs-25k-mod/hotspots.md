# JVM hotspots (JFR ExecutionSample)

Samples: 17191

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.98% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 10 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.16% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 14 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 16 | 0.99% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 17 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.93% | `java.lang.ref.Reference.get() line: 361` |
| 19 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.79% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 22 | 0.70% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 23 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 25 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 27 | 0.62% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 28 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 32 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 33 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 37 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 39 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 40 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5850` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 49 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 51 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1983` |
| 53 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.35% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5107` |
| 57 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 59 | 0.31% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 8 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.14% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 13 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 18 | 0.84% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.80% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.70% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 23 | 0.69% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 26 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6961` |
| 29 | 0.62% | `net.minecraft.world.entity.Mob.baseTick() line: 353` |
| 30 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms

