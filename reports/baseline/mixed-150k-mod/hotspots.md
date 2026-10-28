# JVM hotspots (JFR ExecutionSample)

Samples: 51049

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.64% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 4 | 2.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 2.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 9 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 12 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 13 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 15 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 21 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 24 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.55% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 28 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 31 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 32 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 33 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 39 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 40 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 46 | 0.35% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7365` |
| 47 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 49 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 50 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 54 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 56 | 0.28% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.26% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.24% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.12% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.23% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.89% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 19 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5676` |
| 26 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 29 | 0.58% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |

GC pauses: 0, total 0 ms

