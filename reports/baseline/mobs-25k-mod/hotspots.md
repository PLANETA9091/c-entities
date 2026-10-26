# JVM hotspots (JFR ExecutionSample)

Samples: 16837

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6558` |
| 5 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.36% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 8 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 10 | 1.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 11 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 17 | 0.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.75% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 24 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.57% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 28 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4474` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 34 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 35 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 36 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 37 | 0.45% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 41 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 47 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2672` |
| 51 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 54 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2562` |
| 56 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2499` |
| 60 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 3 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.56% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 8 | 1.47% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5872` |
| 9 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 12 | 1.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5870` |
| 14 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 17 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.87% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.85% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 5978` |
| 27 | 0.74% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 6011` |

GC pauses: 0, total 0 ms

