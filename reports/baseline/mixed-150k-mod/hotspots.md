# JVM hotspots (JFR ExecutionSample)

Samples: 34905

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 2 | 3.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 2.94% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.20% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 21 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 23 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 31 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 35 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 37 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 40 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 43 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 46 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 47 | 0.34% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 48 | 0.34% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 49 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 50 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 52 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 57 | 0.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1309` |
| 60 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 4.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.94% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 6 | 2.20% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 7 | 1.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 8 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 11 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 13 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 21 | 0.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.78% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8091` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |

GC pauses: 0, total 0 ms

