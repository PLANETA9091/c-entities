# JVM hotspots (JFR ExecutionSample)

Samples: 48371

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.00% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 4.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 3 | 4.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 4 | 3.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 5 | 2.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11028` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 7 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 11 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 13 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 15 | 0.91% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.89% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 17 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 23 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.61% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 27 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.54% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.54% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 36 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 38 | 0.47% | `java.util.ArrayList.<init>(int) line: 157` |
| 39 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10940` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7304` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 50 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3179` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 53 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6797` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 56 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3085` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 2 | 8.00% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10575` |
| 5 | 2.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.92% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.40% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.12% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 15 | 0.89% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 16 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 22 | 0.58% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7382` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.53% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |

GC pauses: 0, total 0 ms

