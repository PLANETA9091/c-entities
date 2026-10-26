# JVM hotspots (JFR ExecutionSample)

Samples: 13399

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 5.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.22% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 16 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 1.12% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.88% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.71% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 33 | 0.51% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 36 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 40 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.37% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 42 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 45 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 46 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.31% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.29% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 50 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 51 | 0.28% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 52 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1170` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 56 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 8.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 5.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.24% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.75% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.22% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.08% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.72% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.60% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.54% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 30 | 0.54% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |

GC pauses: 0, total 0 ms

