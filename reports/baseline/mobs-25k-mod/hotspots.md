# JVM hotspots (JFR ExecutionSample)

Samples: 59167

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 60.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4059` |
| 2 | 6.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4131` |
| 3 | 1.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4073` |
| 4 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4058` |
| 5 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4049` |
| 6 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4041` |
| 9 | 0.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11537` |
| 12 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 13 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4040` |
| 18 | 0.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.22% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2158` |
| 20 | 0.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 21 | 0.22% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 23 | 0.21% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11368` |
| 24 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 25 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 26 | 0.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 28 | 0.17% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11190` |
| 29 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.16% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 31 | 0.16% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7755` |
| 32 | 0.15% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 33 | 0.14% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.14% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 35 | 0.14% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.14% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.13% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 12160` |
| 38 | 0.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4057` |
| 39 | 0.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 41 | 0.12% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7675` |
| 42 | 0.11% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7366` |
| 44 | 0.11% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 11217` |
| 45 | 0.11% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7341` |
| 46 | 0.10% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 47 | 0.10% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 49 | 0.10% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 51 | 0.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4091` |
| 52 | 0.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 54 | 0.10% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.10% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7429` |
| 57 | 0.10% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 59 | 0.09% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 60 | 0.09% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 17501` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 72.65% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2158` |
| 2 | 0.93% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 5 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 6 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 7 | 0.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 9 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.32% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 11 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 13 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 14 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 15 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11615` |
| 16 | 0.27% | `java.util.HashMap.get(Object) line: 565` |
| 17 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11706` |
| 18 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 0.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11630` |
| 20 | 0.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11536` |
| 21 | 0.23% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 22 | 0.22% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 23 | 0.22% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.19% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.18% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7341` |
| 28 | 0.18% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

