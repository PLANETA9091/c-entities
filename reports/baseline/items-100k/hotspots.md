# JVM hotspots (JFR ExecutionSample)

Samples: 13998

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 3 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.23% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 8 | 2.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 2.10% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 2.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.12% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.03% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.71% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.64% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.51% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.44% | `java.util.Optional.isPresent() line: 154` |
| 35 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 38 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 41 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.34% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 43 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 44 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 45 | 0.33% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 46 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 49 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.29% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 54 | 0.27% | `java.util.ArrayList.clear() line: 735` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 59 | 0.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 60 | 0.26% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.31% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 5 | 4.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.23% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 8 | 2.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 2.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.09% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 16 | 1.15% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.01% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 19 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 21 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 23 | 0.79% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 24 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 29 | 0.71% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |

GC pauses: 0, total 0 ms

