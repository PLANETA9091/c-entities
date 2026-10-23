# JVM hotspots (JFR ExecutionSample)

Samples: 8733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 4 | 2.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 1.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 8 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 11 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.06% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 19 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 20 | 0.78% | `java.util.Optional.isPresent() line: 154` |
| 21 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 24 | 0.70% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.68% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 28 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 29 | 0.62% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 30 | 0.62% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 31 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.58% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 33 | 0.57% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 35 | 0.56% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 37 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.52% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.49% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 42 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 45 | 0.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 46 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 47 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 48 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.35% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |
| 51 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.33% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 57 | 0.33% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 59 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 3.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.53% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 5 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 8 | 1.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.68% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 12 | 1.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 13 | 1.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 14 | 1.41% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 1.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 17 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 21 | 1.06% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1895` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.94% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 24 | 0.92% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |
| 25 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.88% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 27 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.84% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 30 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |

GC pauses: 0, total 0 ms

