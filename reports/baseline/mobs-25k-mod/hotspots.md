# JVM hotspots (JFR ExecutionSample)

Samples: 17412

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 5.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 4 | 2.15% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 5 | 1.94% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11157` |
| 7 | 1.37% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 8 | 1.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 10 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 16 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 23 | 0.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 24 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4870` |
| 28 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 31 | 0.41% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7234` |
| 32 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.39% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8026` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4933` |
| 35 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5179` |
| 37 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 39 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 40 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4845` |
| 42 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 44 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 45 | 0.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 47 | 0.28% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 48 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 51 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 52 | 0.27% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 53 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.24% | `java.util.EnumMap.get(Object) line: 248` |
| 59 | 0.24% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 5.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 4 | 3.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.15% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2306` |
| 6 | 1.94% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 7 | 1.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.56% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.46% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10704` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 12 | 1.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 16 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 17 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 22 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7402` |
| 23 | 0.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2451` |
| 26 | 0.64% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

