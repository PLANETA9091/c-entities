# JVM hotspots (JFR ExecutionSample)

Samples: 13628

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 3.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.56% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.34% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 17 | 1.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.09% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.96% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.84% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 638` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.69% | `java.util.Optional.isPresent() line: 154` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.66% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 32 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 35 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.48% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 37 | 0.45% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 39 | 0.41% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 40 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.38% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 42 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 45 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 47 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 48 | 0.32% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 53 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 54 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 55 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.25% | `java.util.ArrayList.clear() line: 735` |
| 58 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.24% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 5.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 4 | 5.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 5.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.22% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.22% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.67% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.54% | `java.util.ArrayList.toArray() line: 370` |
| 14 | 1.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 1.28% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 19 | 1.07% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 0.99% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.96% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.86% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.68% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.66% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.61% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |

GC pauses: 0, total 0 ms

