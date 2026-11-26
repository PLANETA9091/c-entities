# JVM hotspots (JFR ExecutionSample)

Samples: 29317

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 49.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4017` |
| 2 | 8.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4089` |
| 3 | 1.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4000` |
| 4 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4007` |
| 6 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4016` |
| 7 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3998` |
| 11 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 15 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11482` |
| 16 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3999` |
| 19 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 20 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11135` |
| 21 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7386` |
| 24 | 0.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 0.31% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5550` |
| 26 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11313` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 29 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 30 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 31 | 0.26% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.25% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 33 | 0.24% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7323` |
| 34 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11560` |
| 35 | 0.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 37 | 0.21% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7632` |
| 38 | 0.20% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 40 | 0.20% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2119` |
| 41 | 0.20% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.19% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 43 | 0.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 44 | 0.18% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 45 | 0.17% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 12105` |
| 47 | 0.16% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7298` |
| 49 | 0.16% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 50 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4031` |
| 53 | 0.15% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.15% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.15% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 56 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 57 | 0.14% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 58 | 0.14% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 59 | 0.14% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 60 | 0.14% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 61.72% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2119` |
| 2 | 1.08% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 5 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 6 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11575` |
| 7 | 0.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 10 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 0.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16836` |
| 14 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11481` |
| 15 | 0.44% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 16 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11560` |
| 17 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 0.40% | `java.util.HashMap.get(Object) line: 565` |
| 19 | 0.36% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11651` |
| 21 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 22 | 0.35% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.33% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 24 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.28% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7633` |
| 29 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

