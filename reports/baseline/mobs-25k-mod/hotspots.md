# JVM hotspots (JFR ExecutionSample)

Samples: 28102

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 48.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3938` |
| 2 | 8.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4010` |
| 3 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3921` |
| 4 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3928` |
| 6 | 0.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3937` |
| 7 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 0.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.57% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3920` |
| 17 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10982` |
| 18 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3919` |
| 19 | 0.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 21 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 22 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10635` |
| 23 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7055` |
| 24 | 0.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 25 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6992` |
| 26 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.26% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5445` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10813` |
| 30 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.24% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.23% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 34 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 35 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7301` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11605` |
| 39 | 0.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 40 | 0.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 41 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.19% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 43 | 0.19% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.19% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11060` |
| 46 | 0.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.17% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 49 | 0.17% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 52 | 0.15% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.15% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16401` |
| 56 | 0.15% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.14% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.14% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 60.69% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2048` |
| 2 | 1.16% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 4 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 8 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.58% | `java.util.HashMap.get(Object) line: 565` |
| 10 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 0.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11075` |
| 13 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11060` |
| 14 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.47% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 19 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10981` |
| 20 | 0.41% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.39% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 22 | 0.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7302` |
| 23 | 0.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11151` |
| 24 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.29% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |

GC pauses: 0, total 0 ms

