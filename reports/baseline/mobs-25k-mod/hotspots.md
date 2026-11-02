# JVM hotspots (JFR ExecutionSample)

Samples: 12615

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 2 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.25% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9949` |
| 4 | 2.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9580` |
| 9 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 0.97% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.80% | `java.lang.ref.Reference.get() line: 361` |
| 16 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 17 | 0.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7301` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4337` |
| 21 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.63% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4490` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4400` |
| 26 | 0.55% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 30 | 0.50% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 31 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9861` |
| 32 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 33 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 35 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.44% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.43% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 40 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 42 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4744` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4667` |
| 46 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 50 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 52 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4312` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.31% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 57 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.11% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 2.05% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 8 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.26% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 1.09% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9511` |
| 18 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.98% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.97% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.82% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 27 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4510` |
| 29 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.79% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4668` |

GC pauses: 0, total 0 ms

