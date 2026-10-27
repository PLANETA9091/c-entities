# JVM hotspots (JFR ExecutionSample)

Samples: 18909

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |
| 5 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 8 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1463` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 11 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6229` |
| 17 | 0.88% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 19 | 0.83% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.74% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.56% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 25 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6425` |
| 29 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 30 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.47% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 32 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1466` |
| 34 | 0.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 35 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7281` |
| 37 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2727` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2617` |
| 41 | 0.41% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 42 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 44 | 0.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 45 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7403` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 51 | 0.36% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 52 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2554` |
| 54 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 56 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4529` |
| 58 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1079` |
| 2 | 4.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1467` |
| 5 | 1.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7208` |
| 7 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 0.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 0.92% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.89% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.85% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 19 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6383` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 2861` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6212` |
| 28 | 0.61% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6214` |
| 29 | 0.60% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.58% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |

GC pauses: 0, total 0 ms

