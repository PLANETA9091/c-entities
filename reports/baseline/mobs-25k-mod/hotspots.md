# JVM hotspots (JFR ExecutionSample)

Samples: 17984

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 5 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 10 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 13 | 0.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.88% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.85% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 18 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 20 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 24 | 0.54% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 25 | 0.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 27 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7670` |
| 29 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 32 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.49% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 35 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 36 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5539` |
| 39 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.39% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2975` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 48 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6692` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2802` |
| 54 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 60 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 4 | 1.67% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 8 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 12 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 0.96% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 14 | 0.88% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.87% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6662` |
| 18 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 21 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.75% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6479` |
| 26 | 0.70% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6481` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3109` |

GC pauses: 0, total 0 ms

