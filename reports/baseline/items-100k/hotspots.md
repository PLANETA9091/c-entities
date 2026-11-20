# JVM hotspots (JFR ExecutionSample)

Samples: 14984

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.32% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 8 | 2.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 1.95% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 16 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 18 | 0.99% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 0.99% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.99% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 0.87% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 27 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 29 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 32 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 33 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 34 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 36 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.43% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 40 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.42% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 45 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 48 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 49 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 50 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.29% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.27% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |
| 55 | 0.27% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 56 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 60 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 6.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 4.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.92% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.43% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 9 | 2.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 1.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.13% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 18 | 1.11% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 1.09% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.87% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.84% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 26 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.61% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |

GC pauses: 0, total 0 ms

