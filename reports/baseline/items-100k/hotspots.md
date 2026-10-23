# JVM hotspots (JFR ExecutionSample)

Samples: 13898

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 3 | 3.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 15 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 17 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.94% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.81% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.78% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 23 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 28 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 30 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 31 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 33 | 0.50% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 34 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 36 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.40% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 42 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 43 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 47 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 50 | 0.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 51 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 52 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 53 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 57 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 59 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 4.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 3.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.15% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.01% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.90% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 14 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 15 | 1.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 17 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.84% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 24 | 0.81% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 26 | 0.79% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 27 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.68% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 29 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |

GC pauses: 0, total 0 ms

