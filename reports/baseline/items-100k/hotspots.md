# JVM hotspots (JFR ExecutionSample)

Samples: 13777

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 6 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.98% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 13 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 16 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.13% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 19 | 0.99% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 20 | 0.99% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 21 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.91% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 29 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 31 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.70% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 34 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 38 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 41 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 43 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 46 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 47 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.35% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 51 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 53 | 0.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 55 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 56 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 58 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 59 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.27% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 5.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.94% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 4.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 7 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.98% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.84% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 11 | 1.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.70% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 14 | 1.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 15 | 1.43% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 16 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 18 | 1.09% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.07% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 21 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 22 | 0.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.99% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 24 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.97% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 26 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.86% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |
| 30 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

