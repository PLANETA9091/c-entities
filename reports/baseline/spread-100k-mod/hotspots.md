# JVM hotspots (JFR ExecutionSample)

Samples: 49549

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 4.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 3 | 4.61% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2669` |
| 4 | 3.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 5 | 2.82% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 6 | 2.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10083` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10706` |
| 10 | 1.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3082` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 12 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3111` |
| 14 | 0.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.80% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 16 | 0.79% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.71% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2400` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3096` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2896` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.58% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 30 | 0.56% | `java.util.ArrayList.<init>(int) line: 157` |
| 31 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10618` |
| 35 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 36 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2409` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.45% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 42 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7249` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.35% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 50 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.33% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6915` |
| 54 | 0.31% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 55 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 59 | 0.26% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2398` |
| 60 | 0.25% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7093` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 3 | 4.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2442` |
| 4 | 2.82% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10253` |
| 6 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.28% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.25% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 0.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.80% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 17 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.73% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 20 | 0.71% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7332` |
| 25 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3041` |
| 28 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |

GC pauses: 0, total 0 ms

