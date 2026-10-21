# JVM hotspots (JFR ExecutionSample)

Samples: 13651

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 1.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 18 | 0.92% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 19 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 20 | 0.78% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 21 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 22 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 25 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.62% | `java.lang.ThreadLocal.get() line: 172` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 31 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.56% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.54% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 37 | 0.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 39 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.40% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 46 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 47 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 48 | 0.35% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 50 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 51 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 53 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 56 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 58 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 59 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 60 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 2.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.34% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 2.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 15 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 17 | 0.94% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.93% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 19 | 0.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 22 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.71% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 28 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |
| 29 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

GC pauses: 0, total 0 ms

