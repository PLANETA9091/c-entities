# JVM hotspots (JFR ExecutionSample)

Samples: 12285

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 10 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 12 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.73% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.67% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 15 | 1.63% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.53% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 18 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 1.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.07% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 23 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 24 | 1.03% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 25 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.84% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 29 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 31 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 34 | 0.52% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 38 | 0.49% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 40 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 44 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 46 | 0.35% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 47 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 49 | 0.33% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 51 | 0.31% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 53 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.29% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 56 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 57 | 0.27% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 58 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 60 | 0.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 2.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.16% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 9 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.09% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 11 | 2.03% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 14 | 1.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.51% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 17 | 1.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.11% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 21 | 0.94% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 23 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.82% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.isInRain() line: 2164` |
| 28 | 0.63% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 30 | 0.59% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |

GC pauses: 0, total 0 ms

