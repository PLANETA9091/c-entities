# JVM hotspots (JFR ExecutionSample)

Samples: 13558

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.81% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.40% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 16 | 1.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 1.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.05% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 1.05% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 23 | 0.83% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.55% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.53% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 33 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 40 | 0.41% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 41 | 0.40% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 48 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 50 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 51 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 54 | 0.29% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 56 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 57 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 60 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.94% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.94% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.80% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.37% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.94% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 21 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.85% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.70% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 30 | 0.65% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |

GC pauses: 0, total 0 ms

