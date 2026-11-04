# JVM hotspots (JFR ExecutionSample)

Samples: 14873

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.73% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.72% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 1.18% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.81% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.75% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 30 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 32 | 0.54% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 33 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 40 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.38% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 47 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 52 | 0.32% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 53 | 0.32% | `java.util.ArrayList.clear() line: 735` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 56 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 57 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 58 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 60 | 0.28% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.44% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.70% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.78% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 12 | 1.72% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.72% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.59% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.05% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 19 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 26 | 0.79% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 27 | 0.78% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 29 | 0.75% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 30 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

