# JVM hotspots (JFR ExecutionSample)

Samples: 13611

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.34% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 2.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.40% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.04% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.99% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.62% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 31 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.51% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 33 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.48% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 37 | 0.48% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 39 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 44 | 0.41% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 45 | 0.38% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 46 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 50 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 51 | 0.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 52 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 58 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 6.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 5.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.34% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 2.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.71% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.48% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.45% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 19 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.89% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 25 | 0.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 26 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.71% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.65% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms

