# JVM hotspots (JFR ExecutionSample)

Samples: 13512

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.06% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.55% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.84% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 29 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.60% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 31 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 35 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.45% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.41% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 45 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 46 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.39% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 48 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.38% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 52 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 54 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 58 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.79% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.89% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.85% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 16 | 1.11% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.04% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.95% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 20 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.82% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 25 | 0.72% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 26 | 0.69% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.62% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |

GC pauses: 0, total 0 ms

