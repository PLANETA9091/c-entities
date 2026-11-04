# JVM hotspots (JFR ExecutionSample)

Samples: 17193

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 4.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 2.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 5 | 1.76% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10676` |
| 7 | 1.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 13 | 0.95% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 16 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 18 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.70% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 23 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4769` |
| 28 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7092` |
| 30 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.47% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7858` |
| 33 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |
| 36 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 37 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 43 | 0.30% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10588` |
| 46 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 50 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6941` |
| 53 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 54 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 59 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 4.06% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 3.52% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 6 | 1.79% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 7 | 1.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 13 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10222` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.92% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7247` |
| 22 | 0.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7404` |
| 29 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms

