# JVM hotspots (JFR ExecutionSample)

Samples: 10218

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 4 | 2.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 6 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 14 | 1.32% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 19 | 1.03% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 20 | 1.01% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 26 | 0.76% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.74% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 31 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.68% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 34 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 35 | 0.64% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.63% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 37 | 0.62% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 40 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 44 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 46 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 51 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 52 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 53 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.35% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 55 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 56 | 0.34% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 58 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.29% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 191` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 3.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.25% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 5 | 2.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.04% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 9 | 2.01% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.57% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 1.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 18 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 21 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.93% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 25 | 0.91% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 26 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 28 | 0.88% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 29 | 0.87% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 30 | 0.85% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |

GC pauses: 0, total 0 ms

