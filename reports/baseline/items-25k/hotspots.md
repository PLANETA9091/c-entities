# JVM hotspots (JFR ExecutionSample)

Samples: 3464

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 2 | 3.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 2.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 1.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 6 | 1.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 1.53% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 12 | 1.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.18% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 14 | 1.18% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.07% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 16 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 19 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 20 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.87% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 22 | 0.84% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 23 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 24 | 0.69% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 25 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 27 | 0.61% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 28 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 30 | 0.58% | `java.lang.String.hashCode() line: 2358` |
| 31 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 32 | 0.55% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 35 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 36 | 0.43% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 37 | 0.43% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 39 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.40% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 41 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 45 | 0.38% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 46 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.35% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 48 | 0.35% | `java.util.Objects.equals(Object, Object) line: 64` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 50 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.35% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 52 | 0.32% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 53 | 0.32% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 54 | 0.32% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 55 | 0.32% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 56 | 0.32% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 57 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 58 | 0.32% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 60 | 0.32% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 2 | 3.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 2.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 2.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 9 | 1.59% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.50% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 16 | 1.21% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 1.10% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 20 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 21 | 1.07% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 22 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 23 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 29 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |

GC pauses: 0, total 0 ms

