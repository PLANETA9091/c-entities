# JVM hotspots (JFR ExecutionSample)

Samples: 46702

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 2 | 7.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 4 | 2.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 5 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 7 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 8 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |
| 10 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3101` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10676` |
| 13 | 1.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.03% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 19 | 0.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 21 | 0.85% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 23 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 25 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3088` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 29 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 34 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.45% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 39 | 0.43% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.34% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 49 | 0.34% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 51 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10092` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10588` |
| 55 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 57 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 2 | 7.24% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 2.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 4 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10222` |
| 8 | 1.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 9 | 1.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.14% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3094` |
| 16 | 0.89% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.88% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 18 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 22 | 0.65% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.49% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

