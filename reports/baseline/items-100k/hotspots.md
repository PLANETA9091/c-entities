# JVM hotspots (JFR ExecutionSample)

Samples: 17255

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 2.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.84% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 1.15% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.59% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.59% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.56% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 35 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 39 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 46 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.32% | `java.util.ArrayList.clear() line: 735` |
| 49 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 50 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 54 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 56 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.27% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 59 | 0.25% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 60 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 3.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 6 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 1.73% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 9 | 1.64% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 15 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 1.12% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 1.08% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.75% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 27 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.72% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms

