# JVM hotspots (JFR ExecutionSample)

Samples: 14109

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.13% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 12 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.41% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 16 | 1.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.22% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.82% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.80% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.70% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.62% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 31 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 34 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.50% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 38 | 0.47% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 44 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 46 | 0.38% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 47 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 48 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 50 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 54 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 55 | 0.30% | `java.util.ArrayList.clear() line: 735` |
| 56 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 60 | 0.28% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 3.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.38% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.08% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.74% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.46% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.09% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 1.05% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 1.04% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 21 | 1.03% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 22 | 0.97% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.84% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 27 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.73% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |

GC pauses: 0, total 0 ms

