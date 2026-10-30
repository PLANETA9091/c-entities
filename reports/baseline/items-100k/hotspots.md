# JVM hotspots (JFR ExecutionSample)

Samples: 14186

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 5.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.19% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 23 | 0.96% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.95% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.87% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 29 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.60% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 32 | 0.49% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 33 | 0.47% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 37 | 0.43% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 38 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 42 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 46 | 0.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 49 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 52 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 53 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 55 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 56 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 57 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 58 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 60 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.26% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 10 | 2.12% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.90% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.64% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.25% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 0.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 0.94% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.94% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 24 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 25 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |

GC pauses: 0, total 0 ms

