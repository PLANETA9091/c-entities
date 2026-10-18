# JVM hotspots (JFR ExecutionSample)

Samples: 13107

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 5 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 1.41% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 7 | 1.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.21% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.17% | `java.lang.ThreadLocal.get() line: 172` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 15 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 17 | 0.89% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 0.85% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 20 | 0.83% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 25 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 26 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer$Data.moonrise$getPalette() line: 458` |
| 27 | 0.63% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 29 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 31 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 33 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 34 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 35 | 0.53% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 36 | 0.52% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 37 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 39 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 40 | 0.49% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 41 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 45 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 48 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 49 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 50 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.38% | `java.util.HashMap.getNode(Object) line: 585` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 53 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.34% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 56 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 59 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 60 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.89% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.37% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.30% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 11 | 1.21% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 14 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 0.97% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 17 | 0.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 21 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 24 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 91` |
| 27 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 29 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |

GC pauses: 0, total 0 ms

