# JVM hotspots (JFR ExecutionSample)

Samples: 14634

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.27% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.74% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.74% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 16 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 1.20% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 1.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.72% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 35 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 37 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.39% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 43 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.34% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 47 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 51 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 54 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 55 | 0.27% | `java.util.ArrayList$Itr.hasNext() line: 1045` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 57 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 60 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.92% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.53% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 2.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.87% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.78% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 14 | 1.59% | `java.util.ArrayList.toArray() line: 370` |
| 15 | 1.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.90% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.72% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.66% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |

GC pauses: 0, total 0 ms

