# JVM hotspots (JFR ExecutionSample)

Samples: 11522

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 5 | 3.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 9 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.87% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 1.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 17 | 1.41% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 1.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 21 | 1.19% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 22 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 24 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.99% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 29 | 0.63% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 31 | 0.59% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 34 | 0.48% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 35 | 0.48% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 36 | 0.45% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 37 | 0.43% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.getXRot() line: 5291` |
| 39 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.39% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 41 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 42 | 0.38% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 43 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 48 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 49 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 51 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.28% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 3.75% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 2.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 8 | 2.64% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.02% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.98% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 13 | 1.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 14 | 1.42% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 15 | 1.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 16 | 1.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.37% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.32% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 20 | 0.97% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 22 | 0.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 23 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 25 | 0.82% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 26 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.71% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 28 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |

GC pauses: 0, total 0 ms

