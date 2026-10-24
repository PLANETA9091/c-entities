# JVM hotspots (JFR ExecutionSample)

Samples: 12639

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 4.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 1.65% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 17 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.17% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 19 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.17% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 1.12% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 23 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 0.96% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 26 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 27 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 31 | 0.66% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 35 | 0.49% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 37 | 0.41% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 39 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 42 | 0.38% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 43 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 45 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 47 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 48 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.26% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 59 | 0.25% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 7.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 5.32% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 2.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.08% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.84% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 13 | 1.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 14 | 1.52% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.35% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 17 | 1.34% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 18 | 1.09% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.87% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 21 | 0.85% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.85% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 24 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.isInRain() line: 2164` |
| 30 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

GC pauses: 0, total 0 ms

