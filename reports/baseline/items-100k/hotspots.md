# JVM hotspots (JFR ExecutionSample)

Samples: 11891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.09% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 6 | 2.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.96% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 14 | 1.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.63% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.61% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 1.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 1.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 1.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 1.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 1.15% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 1.02% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 35 | 0.54% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 36 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 39 | 0.46% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.39% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 44 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 46 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 49 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.29% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.29% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2415` |
| 56 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.29% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 5.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 3.33% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 3.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.72% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.20% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.56% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 16 | 1.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 1.33% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.19% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 1.03% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 26 | 0.71% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 27 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |

GC pauses: 0, total 0 ms

