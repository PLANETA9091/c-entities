# JVM hotspots (JFR ExecutionSample)

Samples: 3921

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 3.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 2.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.19% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.84% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.40% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 11 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.17% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 16 | 1.12% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 19 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 1.07% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 21 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 23 | 0.82% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 24 | 0.77% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.69% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 27 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.64% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.64% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 30 | 0.61% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 31 | 0.59% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 32 | 0.59% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 33 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 34 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 35 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 37 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 38 | 0.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 39 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 40 | 0.38% | `java.lang.ThreadLocal.get() line: 172` |
| 41 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 42 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 43 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 46 | 0.36% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 47 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 48 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 49 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.33% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 52 | 0.33% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.noise(double, double, double, double, double) line: 61` |
| 53 | 0.33% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 55 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 56 | 0.31% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 59 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 60 | 0.31% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 3.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 2.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 8 | 2.19% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.84% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.40% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 13 | 1.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 16 | 1.15% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 20 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 21 | 1.07% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 1.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 24 | 0.99% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 25 | 0.87% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 27 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 28 | 0.66% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.64% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

