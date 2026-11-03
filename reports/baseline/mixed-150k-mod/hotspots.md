# JVM hotspots (JFR ExecutionSample)

Samples: 45098

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 2 | 12.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.50% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 4 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.13% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 6 | 2.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2244` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2483` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2255` |
| 15 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.10% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 19 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10402` |
| 24 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7090` |
| 39 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 41 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2542` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6756` |
| 44 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 45 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9859` |
| 47 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 51 | 0.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9862` |
| 54 | 0.22% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.22% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10298` |
| 57 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.22% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.21% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 60 | 0.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2288` |
| 2 | 12.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 3 | 3.93% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 6 | 2.15% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 7 | 2.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2509` |
| 9 | 1.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 10 | 1.82% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 11 | 1.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2568` |
| 14 | 1.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.08% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 1.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.64% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7173` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7576` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 29 | 0.44% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

