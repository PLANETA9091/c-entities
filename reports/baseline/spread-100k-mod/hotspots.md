# JVM hotspots (JFR ExecutionSample)

Samples: 42151

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.50% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 3.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 5 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 7 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 8 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 12 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.20% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 1.10% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10950` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 20 | 0.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.93% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 23 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 31 | 0.70% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.63% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.61% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 36 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3097` |
| 38 | 0.58% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.54% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 41 | 0.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 42 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 44 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 49 | 0.40% | `java.util.ArrayList.<init>(int) line: 157` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 52 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 55 | 0.36% | `java.util.EnumMap.get(Object) line: 248` |
| 56 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10862` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3179` |
| 60 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 2 | 7.50% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 5 | 3.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10497` |
| 6 | 2.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.63% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 10 | 1.45% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.20% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 13 | 1.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 1.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 19 | 0.93% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 22 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.63% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

