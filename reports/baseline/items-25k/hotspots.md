# JVM hotspots (JFR ExecutionSample)

Samples: 3303

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 3 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.03% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 5 | 2.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 10 | 1.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 1.27% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.06% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 17 | 1.03% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 18 | 0.97% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 19 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.85% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 22 | 0.85% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 23 | 0.85% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 24 | 0.79% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 26 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 28 | 0.67% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 29 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 30 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 31 | 0.58% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 32 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 33 | 0.58% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 34 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 36 | 0.54% | `java.util.EnumMap.get(Object) line: 247` |
| 37 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 38 | 0.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 39 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 41 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 42 | 0.45% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 44 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 45 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 46 | 0.42% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 47 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 49 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.39% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 51 | 0.39% | `java.util.HashMap.getNode(Object) line: 585` |
| 52 | 0.39% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 54 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 55 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 56 | 0.36% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 57 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 58 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 59 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 60 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 2.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 2.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 2.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 2.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 2.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 8 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 10 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 14 | 1.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 17 | 1.21% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 18 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 21 | 1.00% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 22 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 23 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 24 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 25 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 26 | 0.79% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 28 | 0.76% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 29 | 0.73% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |

GC pauses: 0, total 0 ms

