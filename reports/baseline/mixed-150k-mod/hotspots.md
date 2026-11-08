# JVM hotspots (JFR ExecutionSample)

Samples: 31747

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.49% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 4 | 3.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 5 | 2.50% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 6 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 12 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 15 | 1.22% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 1.11% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 25 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 32 | 0.64% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 33 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 34 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 36 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 41 | 0.44% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.41% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 48 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 50 | 0.39% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 11291` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7085` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 54 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 56 | 0.33% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 57 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 59 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 4 | 3.49% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 5 | 3.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.50% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 7 | 2.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 11 | 1.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 13 | 1.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 16 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 1.11% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 18 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 1.04% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8091` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.77% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 30 | 0.74% | `java.util.HashMap.get(Object) line: 565` |

GC pauses: 0, total 0 ms

