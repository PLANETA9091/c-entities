# JVM hotspots (JFR ExecutionSample)

Samples: 2550

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 2.94% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.90% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 2.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.92% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 9 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 12 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.80% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 14 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 16 | 1.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 17 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.22% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 20 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.90% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 22 | 0.86% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 24 | 0.75% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 25 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.71% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 27 | 0.67% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1123` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 30 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 31 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 32 | 0.59% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 33 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.55% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 36 | 0.51% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 37 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 39 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 40 | 0.47% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 42 | 0.47% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 43 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 487` |
| 46 | 0.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 47 | 0.39% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 48 | 0.39% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 49 | 0.39% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 50 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 51 | 0.39% | `net.minecraft.world.entity.Entity.getTicksFrozen() line: 3632` |
| 52 | 0.35% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 53 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 54 | 0.35% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 55 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 56 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 59 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 4.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 3 | 3.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.94% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 2.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 7 | 2.67% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 9 | 2.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 10 | 2.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 2.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 12 | 1.84% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.41% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 16 | 1.25% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 1.22% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 19 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 20 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 1.10% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 25 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 27 | 0.82% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1275` |
| 28 | 0.78% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 29 | 0.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |

GC pauses: 0, total 0 ms

