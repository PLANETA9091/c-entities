# JVM hotspots (JFR ExecutionSample)

Samples: 15877

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 2.33% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.98% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.18% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.08% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 26 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 27 | 0.81% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.72% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 30 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 31 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 34 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 38 | 0.42% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 39 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 41 | 0.38% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 42 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 43 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 49 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 50 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 54 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 56 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.26% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 3.04% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 2.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 2.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.42% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 11 | 2.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 14 | 1.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 1.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.99% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.95% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 20 | 0.87% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 21 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 23 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.80% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.65% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

