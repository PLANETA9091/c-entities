# JVM hotspots (JFR ExecutionSample)

Samples: 15425

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.73% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 16 | 1.30% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 1.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.07% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.86% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.85% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.65% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 32 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.50% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.47% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 36 | 0.46% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 37 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 40 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 41 | 0.38% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 42 | 0.37% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 44 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 45 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.29% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 48 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 49 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 51 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.26% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 54 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 56 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 57 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 58 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 60 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.07% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.98% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 16 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 0.93% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 26 | 0.67% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 27 | 0.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 28 | 0.64% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms

