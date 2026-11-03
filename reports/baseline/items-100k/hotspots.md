# JVM hotspots (JFR ExecutionSample)

Samples: 14446

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.04% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 1.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.20% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 1.17% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.72% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 27 | 0.64% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.51% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 35 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.48% | `java.util.Optional.isPresent() line: 154` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 41 | 0.44% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.41% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 44 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 48 | 0.35% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 49 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 51 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 53 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 54 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 56 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 59 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.26% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 3.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.62% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.04% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.82% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.74% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.64% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.32% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.10% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 19 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.98% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.93% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 25 | 0.82% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 26 | 0.82% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 27 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.73% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

