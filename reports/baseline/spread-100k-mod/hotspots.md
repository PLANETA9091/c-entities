# JVM hotspots (JFR ExecutionSample)

Samples: 48251

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 5.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 3 | 4.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 4 | 3.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 6 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2957` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2928` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 13 | 0.92% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 0.82% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 0.81% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 16 | 0.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.75% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 23 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2942` |
| 34 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7090` |
| 37 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.47% | `java.util.ArrayList.<init>(int) line: 157` |
| 39 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 41 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2935` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2255` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10402` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6756` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2941` |
| 50 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.31% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 52 | 0.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 54 | 0.28% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 58 | 0.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4680` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7158` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 2 | 8.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 3 | 5.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2288` |
| 4 | 3.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 9 | 1.41% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 10 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.82% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.81% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 17 | 0.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7173` |
| 21 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7158` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.56% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7089` |
| 30 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |

GC pauses: 0, total 0 ms

