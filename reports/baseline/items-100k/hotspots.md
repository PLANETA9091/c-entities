# JVM hotspots (JFR ExecutionSample)

Samples: 12814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 2.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 15 | 1.34% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.22% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 1.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 1.08% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 23 | 1.01% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 24 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.97% | `java.util.Optional.isPresent() line: 154` |
| 27 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 28 | 0.91% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 29 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.67% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2415` |
| 32 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 33 | 0.64% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.47% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 38 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 42 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 44 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.31% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.30% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 55 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.30% | `net.minecraft.server.MinecraftServer.moonrise$executeMidTickTasks() line: 462` |
| 57 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 60 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 5 | 4.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 3.36% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 10 | 2.06% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 2.01% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 2.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.58% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.26% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.22% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 18 | 1.17% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 19 | 1.15% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 23 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.85% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 26 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 28 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.setOldRot() line: 2405` |

GC pauses: 0, total 0 ms

