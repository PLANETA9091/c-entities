# JVM hotspots (JFR ExecutionSample)

Samples: 45695

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 3 | 4.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 4 | 3.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 6 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10677` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |
| 11 | 1.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 12 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10075` |
| 14 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 21 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 27 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 29 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 35 | 0.51% | `java.util.ArrayList.<init>(int) line: 157` |
| 36 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 42 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.44% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 44 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.44% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 46 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10589` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 50 | 0.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 51 | 0.33% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 53 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 57 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 2 | 6.76% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 4.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 4 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 6 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.31% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 12 | 1.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 21 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3094` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 26 | 0.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |

GC pauses: 0, total 0 ms

