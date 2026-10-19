# JVM hotspots (JFR ExecutionSample)

Samples: 15755

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.64% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.21% | `java.lang.ThreadLocal.get() line: 172` |
| 8 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.19% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 16 | 1.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 18 | 0.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 19 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.83% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 22 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 23 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 24 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 27 | 0.70% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 31 | 0.58% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.55% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 36 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 40 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 43 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.40% | `java.util.HashMap.getNode(Object) line: 585` |
| 46 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 47 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.37% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 49 | 0.35% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 50 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 51 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 52 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 53 | 0.31% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 54 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 57 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 59 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 2.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.21% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 13 | 1.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 14 | 1.10% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 16 | 1.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 0.98% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 0.83% | `java.util.EnumMap.get(Object) line: 247` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 25 | 0.72% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 27 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 29 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |

GC pauses: 0, total 0 ms

