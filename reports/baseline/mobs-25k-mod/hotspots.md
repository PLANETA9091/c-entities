# JVM hotspots (JFR ExecutionSample)

Samples: 16458

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 8 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 10 | 0.98% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 0.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.92% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 13 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |
| 16 | 0.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6179` |
| 18 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 24 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1433` |
| 26 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.55% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 30 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.51% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 32 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 35 | 0.47% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.41% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6375` |
| 44 | 0.40% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 45 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4479` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2504` |
| 48 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2567` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2677` |
| 53 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5222` |
| 57 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 60 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 4 | 1.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1418` |
| 5 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.17% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 1.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7153` |
| 14 | 0.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 16 | 0.92% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4010` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7133` |
| 28 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 2811` |

GC pauses: 0, total 0 ms

