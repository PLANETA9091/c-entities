# JVM hotspots (JFR ExecutionSample)

Samples: 17059

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 4.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11144` |
| 6 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 8 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 11 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 14 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 0.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 23 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8026` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7234` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 30 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4870` |
| 34 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4933` |
| 35 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.35% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7083` |
| 37 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 39 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 42 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11056` |
| 43 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 46 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.29% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 52 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 54 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.27% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 56 | 0.27% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.26% | `java.util.EnumMap.get(Object) line: 248` |
| 59 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 4.47% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 3.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 3.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 5 | 2.82% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2306` |
| 6 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 7 | 1.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 11 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10691` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 16 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7402` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.74% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7572` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.66% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

