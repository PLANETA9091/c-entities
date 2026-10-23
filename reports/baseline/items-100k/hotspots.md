# JVM hotspots (JFR ExecutionSample)

Samples: 14390

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 2.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 2.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 2.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.83% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.72% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 13 | 1.69% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.45% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 15 | 1.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 16 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 17 | 1.23% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 18 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 19 | 1.23% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 20 | 1.22% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 22 | 1.04% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 24 | 0.85% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 25 | 0.85% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 26 | 0.82% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 28 | 0.75% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 29 | 0.73% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 31 | 0.69% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 32 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 33 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 34 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 35 | 0.61% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1271` |
| 36 | 0.60% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 449` |
| 37 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 38 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 39 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 40 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 41 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadWrite.setRelease(VarHandle, Object, Object) line: 172` |
| 42 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 43 | 0.54% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.51% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 45 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 46 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 47 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 48 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 49 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 50 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 51 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 52 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 53 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 54 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 55 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 57 | 0.37% | `java.util.HashMap.getNode(Object) line: 585` |
| 58 | 0.36% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 59 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.34% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 4.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 3.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 3.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 2.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 11 | 1.74% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 12 | 1.70% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 16 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.31% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 1.22% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 20 | 1.19% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 21 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 22 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 23 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 1.06% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 26 | 1.05% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 27 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 28 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.97% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 30 | 0.88% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |

GC pauses: 0, total 0 ms

