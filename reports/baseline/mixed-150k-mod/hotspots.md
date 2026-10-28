# JVM hotspots (JFR ExecutionSample)

Samples: 53205

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 4 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7797` |
| 8 | 1.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4716` |
| 10 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 13 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4571` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |
| 16 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 22 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 31 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7742` |
| 32 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.42% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 36 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2703` |
| 37 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.37% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6690` |
| 39 | 0.34% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 40 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7617` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 44 | 0.33% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 46 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 49 | 0.29% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 50 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7299` |
| 51 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.27% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7750` |
| 53 | 0.27% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.23% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.96% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7652` |
| 10 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7644` |
| 21 | 0.62% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.60% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 23 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5610` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.54% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.52% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms

