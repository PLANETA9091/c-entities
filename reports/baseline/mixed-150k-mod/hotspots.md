# JVM hotspots (JFR ExecutionSample)

Samples: 53612

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 4 | 2.52% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 5 | 2.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 7 | 1.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4636` |
| 14 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.67% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7807` |
| 20 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.53% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.51% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 29 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 31 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 34 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 36 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 38 | 0.44% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 40 | 0.35% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 41 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 42 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 43 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7682` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4786` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 55 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6755` |
| 57 | 0.30% | `java.lang.ref.Reference.get() line: 361` |
| 58 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 59 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 60 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.72% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7717` |
| 10 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 12 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7709` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5675` |
| 27 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4783` |

GC pauses: 0, total 0 ms

