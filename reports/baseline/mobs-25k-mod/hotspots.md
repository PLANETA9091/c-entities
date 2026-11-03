# JVM hotspots (JFR ExecutionSample)

Samples: 17525

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 2 | 3.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 4 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 6 | 1.42% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 7 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2244` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 1.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7090` |
| 18 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.66% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6756` |
| 22 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7158` |
| 25 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 27 | 0.50% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 28 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 29 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7700` |
| 31 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4616` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6934` |
| 36 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 38 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4679` |
| 41 | 0.35% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4925` |
| 42 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 44 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5005` |
| 46 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 49 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10402` |
| 50 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 52 | 0.28% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 53 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.26% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 56 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 57 | 0.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.25% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4591` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2288` |
| 2 | 3.91% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 3.68% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 4 | 3.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2509` |
| 8 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.43% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2568` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7173` |
| 12 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7158` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 17 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.03% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 19 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7089` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

