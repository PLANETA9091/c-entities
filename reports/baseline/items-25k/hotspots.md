# JVM hotspots (JFR ExecutionSample)

Samples: 3974

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.75% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 3.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 2.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 9 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 12 | 1.21% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 13 | 1.16% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 14 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 1.08% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.88% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 21 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.81% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 23 | 0.78% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 25 | 0.68% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 28 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 29 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 30 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 31 | 0.48% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 32 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 33 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.43% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 36 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 37 | 0.40% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 38 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.38% | `java.lang.String.hashCode() line: 2358` |
| 40 | 0.38% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 42 | 0.33% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 43 | 0.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 194` |
| 44 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 45 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 46 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 47 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 48 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 49 | 0.33% | `java.util.Arrays.fill(int[], int, int, int) line: 3204` |
| 50 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.30% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 53 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 54 | 0.30% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 55 | 0.30% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 56 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 57 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 58 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 60 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 4.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 3.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 2.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.29% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 2.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.41% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 13 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.31% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.23% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 16 | 1.21% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 17 | 1.16% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 18 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 19 | 0.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 21 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 22 | 0.73% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.73% | `java.util.ArrayList.toArray() line: 369` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 27 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |

GC pauses: 0, total 0 ms

