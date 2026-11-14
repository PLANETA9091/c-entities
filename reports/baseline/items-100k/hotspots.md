# JVM hotspots (JFR ExecutionSample)

Samples: 13122

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 9 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.78% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.30% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.06% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.98% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 21 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.94% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.91% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.66% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 30 | 0.66% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 34 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.45% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 41 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.35% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 46 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 47 | 0.34% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 48 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.commonTick() line: 887` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 51 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 54 | 0.27% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 55 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 58 | 0.25% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.compareAndSet(VarHandle, Object, int, int) line: 178` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 60 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 4.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 2.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.64% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 10 | 2.16% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 13 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.22% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 1.00% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.93% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.77% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.66% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.66% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 29 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 30 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

