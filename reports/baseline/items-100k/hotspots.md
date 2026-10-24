# JVM hotspots (JFR ExecutionSample)

Samples: 12469

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.56% | `java.util.ArrayList$Itr.<init>(ArrayList) line: 1042` |
| 2 | 7.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 4.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.59% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 15 | 1.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 1.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 21 | 0.99% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 24 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 28 | 0.68% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 30 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 31 | 0.43% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 32 | 0.43% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 33 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 34 | 0.43% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 36 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 37 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 41 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 45 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 46 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 48 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 49 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 50 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2405` |
| 52 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.23% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 57 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.22% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 60 | 0.22% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.57% | `java.util.ArrayList.iterator() line: 1030` |
| 2 | 7.47% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 7.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 6.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.91% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 2.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 9 | 1.84% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.72% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 12 | 1.62% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 13 | 1.58% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 14 | 1.38% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 15 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.32% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.08% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 19 | 0.99% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 23 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 24 | 0.71% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 26 | 0.68% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |

GC pauses: 0, total 0 ms

