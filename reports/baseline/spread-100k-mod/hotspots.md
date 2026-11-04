# JVM hotspots (JFR ExecutionSample)

Samples: 47437

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.98% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 4.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 3 | 4.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 4 | 3.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 5 | 2.89% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10678` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10076` |
| 12 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.94% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 0.94% | `java.util.ArrayDeque.addFirst(Object) line: 286` |
| 15 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 18 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 23 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 26 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 28 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 30 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 35 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.50% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.48% | `java.util.ArrayList.<init>(int) line: 157` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10590` |
| 40 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3177` |
| 42 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 45 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.40% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 50 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.31% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 57 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 59 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 2 | 7.98% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 4.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 4 | 2.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |
| 9 | 1.31% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 10 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.06% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 0.94% | `java.util.ArrayDeque.push(Object) line: 577` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 20 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.53% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.53% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 29 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

