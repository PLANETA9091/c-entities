# JVM hotspots (JFR ExecutionSample)

Samples: 10124

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 3 | 2.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 4 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 5 | 2.05% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 9 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4342` |
| 19 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.88% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.85% | `java.lang.ref.Reference.get() line: 361` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.73% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 29 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4279` |
| 32 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7169` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.52% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 37 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.45% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 40 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 43 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4609` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4432` |
| 49 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 51 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 54 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 55 | 0.31% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9202` |
| 57 | 0.28% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 58 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9129` |
| 2 | 2.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 3 | 2.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 1.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 8 | 1.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.25% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3539` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 18 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5935` |
| 24 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4610` |
| 26 | 0.85% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4452` |
| 27 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.82% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.78% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |

GC pauses: 0, total 0 ms

