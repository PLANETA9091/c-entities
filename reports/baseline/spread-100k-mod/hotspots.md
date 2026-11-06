# JVM hotspots (JFR ExecutionSample)

Samples: 40720

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 2 | 5.04% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 4 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 6 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11157` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 10 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3211` |
| 12 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 17 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3202` |
| 19 | 0.90% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 21 | 0.83% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3278` |
| 31 | 0.53% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 36 | 0.48% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 38 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3184` |
| 41 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 43 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2996` |
| 46 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 51 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 54 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10558` |
| 60 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 2 | 5.04% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10704` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 6 | 1.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 12 | 1.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 0.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 15 | 0.94% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2306` |
| 18 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 22 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 29 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

