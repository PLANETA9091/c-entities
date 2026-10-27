# JVM hotspots (JFR ExecutionSample)

Samples: 13533

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 2.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 18 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 23 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.79% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.71% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 32 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 34 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.49% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 37 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 38 | 0.47% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 41 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 42 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 43 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 45 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 46 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 49 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 52 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 56 | 0.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 59 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.30% | `java.util.ArrayList.clear() line: 735` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 3.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.89% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.88% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 1.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.26% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.25% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 17 | 1.17% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 1.08% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.98% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 21 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 22 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.91% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.84% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 27 | 0.83% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 28 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.71% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |

GC pauses: 0, total 0 ms

