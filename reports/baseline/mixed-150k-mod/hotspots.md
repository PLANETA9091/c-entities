# JVM hotspots (JFR ExecutionSample)

Samples: 45005

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2657` |
| 2 | 10.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2388` |
| 4 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10632` |
| 7 | 1.82% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6725` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2386` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10030` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2397` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2625` |
| 14 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5962` |
| 17 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 19 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10544` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.54% | `java.lang.String.hashCode() line: 2433` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3087` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7232` |
| 40 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 42 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 44 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.32% | `java.util.ArrayList.elementData(int) line: 412` |
| 46 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3125` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10004` |
| 49 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6738` |
| 51 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2884` |
| 52 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.26% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 54 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4456` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 57 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10001` |
| 58 | 0.25% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10440` |
| 60 | 0.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2430` |
| 2 | 10.72% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4676` |
| 3 | 4.39% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10178` |
| 6 | 2.17% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 9 | 1.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2469` |
| 10 | 1.84% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 11 | 1.49% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2710` |
| 13 | 1.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.08% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.64% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.56% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.54% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7315` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7300` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7718` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

