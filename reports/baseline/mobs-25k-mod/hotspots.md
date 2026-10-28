# JVM hotspots (JFR ExecutionSample)

Samples: 16860

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7930` |
| 5 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 13 | 1.17% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.86% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 18 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.72% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.71% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 24 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5676` |
| 26 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 28 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 32 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 34 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3052` |
| 36 | 0.49% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 38 | 0.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5799` |
| 40 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.44% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 44 | 0.43% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 46 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 48 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 53 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 54 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 56 | 0.34% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6952` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.30% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 3 | 2.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 4 | 2.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.46% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 11 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 13 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.getSharedFlag$(Entity, int) line: 6074` |
| 17 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.74% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 26 | 0.68% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.66% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4587` |
| 30 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

