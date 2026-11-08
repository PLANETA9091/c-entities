# JVM hotspots (JFR ExecutionSample)

Samples: 30830

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 3.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 3.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 5 | 1.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.60% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 17 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 21 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.75% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 29 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.69% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 31 | 0.67% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 32 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.55% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 40 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 41 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 43 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 45 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.37% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 51 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 54 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 11291` |
| 56 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 60 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 3 | 4.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 1.76% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 8 | 1.60% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 11 | 1.33% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 12 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 15 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.02% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.93% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |

GC pauses: 0, total 0 ms

