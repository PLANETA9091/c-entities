# JVM hotspots (JFR ExecutionSample)

Samples: 13021

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 2.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.93% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.74% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.66% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 17 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 1.08% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 1.04% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.93% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 25 | 0.91% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.62% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 34 | 0.53% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 36 | 0.48% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 37 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 39 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 41 | 0.39% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 43 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 46 | 0.34% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 47 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 54 | 0.28% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 56 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 58 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.21% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.17% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.14% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.14% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.14% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 19 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 1.09% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 21 | 1.06% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.73% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 24 | 0.65% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 29 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 30 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

GC pauses: 0, total 0 ms

