# JVM hotspots (JFR ExecutionSample)

Samples: 14354

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 16 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.09% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.89% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.79% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.68% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.62% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 36 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.52% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 38 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 44 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 49 | 0.34% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 51 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.30% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 56 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 59 | 0.28% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 60 | 0.26% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.51% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 2.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.15% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.03% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 2.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.75% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 13 | 1.73% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 14 | 1.58% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.35% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 1.03% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 22 | 0.93% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.91% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.89% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.83% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 28 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.73% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

