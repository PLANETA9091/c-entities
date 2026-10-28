# JVM hotspots (JFR ExecutionSample)

Samples: 48011

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 4 | 2.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 1.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 10 | 1.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.69% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 20 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.65% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 22 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 25 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 29 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 33 | 0.53% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 35 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 42 | 0.47% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 43 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 45 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.44% | `java.util.ImmutableCollections$MapN$MapNIterator.next() line: 1540` |
| 47 | 0.44% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 55 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 2.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.32% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.30% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 0.99% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 18 | 0.86% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.70% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 23 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

