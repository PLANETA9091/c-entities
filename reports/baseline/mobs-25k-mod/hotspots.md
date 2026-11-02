# JVM hotspots (JFR ExecutionSample)

Samples: 12915

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.01% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9605` |
| 4 | 1.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9269` |
| 9 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 12 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 13 | 0.89% | `java.lang.ref.Reference.get() line: 361` |
| 14 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.84% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 17 | 0.82% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 18 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9672` |
| 20 | 0.67% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 21 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.62% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4456` |
| 24 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4366` |
| 27 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 31 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 32 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4303` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7196` |
| 35 | 0.48% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 38 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.41% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4710` |
| 42 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 43 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 45 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.36% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 49 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 50 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4633` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4278` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9550` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 2 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.01% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 12 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.00% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.94% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 21 | 0.89% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4476` |
| 22 | 0.82% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 23 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9257` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9265` |
| 28 | 0.74% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 29 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 30 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

