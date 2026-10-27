# JVM hotspots (JFR ExecutionSample)

Samples: 14421

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.91% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 5 | 2.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.09% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.95% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.37% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 1.01% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.90% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.72% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 32 | 0.55% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.53% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 35 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 40 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 43 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 44 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.35% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 46 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.30% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 347` |
| 52 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 55 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 58 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 60 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.91% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 7 | 2.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.10% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 10 | 1.84% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.31% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 16 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.03% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 19 | 1.01% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.90% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 22 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.87% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 24 | 0.85% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 25 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 27 | 0.78% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.73% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms

