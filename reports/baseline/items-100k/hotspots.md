# JVM hotspots (JFR ExecutionSample)

Samples: 19199

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 4 | 3.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.24% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 17 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.08% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.83% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.70% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.59% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 28 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.57% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 30 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 32 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 34 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 36 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 40 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 47 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.28% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 52 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 53 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 58 | 0.23% | `java.util.ArrayList.clear() line: 735` |
| 59 | 0.23% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 4.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 4.23% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 6 | 4.17% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.02% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 9 | 1.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 10 | 1.54% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.23% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 17 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 1.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.83% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.80% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.73% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 27 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.72% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.70% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.66% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |

GC pauses: 0, total 0 ms

