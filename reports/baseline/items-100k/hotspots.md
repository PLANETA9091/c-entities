# JVM hotspots (JFR ExecutionSample)

Samples: 16924

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 2.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 2.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.51% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.51% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 1.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.43% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 18 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 20 | 1.12% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 1.02% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 22 | 0.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.96% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 0.90% | `java.util.Optional.isPresent() line: 154` |
| 26 | 0.89% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 27 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 29 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 30 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.62% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 34 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.getXRot() line: 5292` |
| 37 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 39 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 41 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 43 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 46 | 0.38% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.35% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 49 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 50 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 55 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 58 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 3.47% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 8 | 2.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.15% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 2.08% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 1.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.49% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 16 | 1.17% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 17 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 20 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 22 | 0.86% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 24 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 25 | 0.79% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 26 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 28 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

