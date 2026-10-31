# JVM hotspots (JFR ExecutionSample)

Samples: 14797

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.99% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.41% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 1.27% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.93% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.89% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.60% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 32 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 33 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 36 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 37 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 39 | 0.45% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 41 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 43 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 48 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.30% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 56 | 0.28% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 59 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 60 | 0.26% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.75% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.97% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 14 | 1.47% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.11% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.07% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 19 | 1.03% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 1.01% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 22 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.89% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 27 | 0.83% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

