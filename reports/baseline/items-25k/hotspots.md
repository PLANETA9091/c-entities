# JVM hotspots (JFR ExecutionSample)

Samples: 3027

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 2 | 3.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 3.70% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 3.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.54% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.05% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 9 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.72% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 16 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 17 | 1.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 18 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 20 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 23 | 0.76% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.76% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 25 | 0.76% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 26 | 0.73% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 27 | 0.73% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 28 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 30 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 33 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 34 | 0.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 35 | 0.53% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 36 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 38 | 0.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 39 | 0.46% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 40 | 0.46% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 41 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 42 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 43 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.46% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 45 | 0.43% | `java.util.ArrayDeque.size() line: 648` |
| 46 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.40% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 49 | 0.36% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 50 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 57 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.30% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 59 | 0.30% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 60 | 0.30% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 5.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 4.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 2.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 2.54% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.05% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 10 | 1.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 11 | 1.95% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 1.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 13 | 1.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.35% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 18 | 1.22% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 19 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.read0() line: 228` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1275` |
| 25 | 0.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 26 | 0.76% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 27 | 0.73% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |

GC pauses: 0, total 0 ms

