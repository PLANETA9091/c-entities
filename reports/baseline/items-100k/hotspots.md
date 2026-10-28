# JVM hotspots (JFR ExecutionSample)

Samples: 14045

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.56% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.10% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 21 | 1.00% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.89% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 31 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 32 | 0.61% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 40 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 43 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 45 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 48 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 49 | 0.33% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 50 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 51 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 53 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 55 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 56 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 58 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 59 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.44% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.91% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.69% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 12 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.56% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 16 | 1.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.15% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 19 | 1.08% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 21 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 23 | 0.98% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.97% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.94% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 26 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.89% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 28 | 0.84% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.83% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |

GC pauses: 0, total 0 ms

