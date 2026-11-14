# JVM hotspots (JFR ExecutionSample)

Samples: 11456

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 2 | 6.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 3 | 2.75% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 9 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12739` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.getFluidHeight(TagKey) line: 176` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 14 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.92% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5817` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.computeSpeed() line: 966` |
| 19 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5881` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5318` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7200` |
| 24 | 0.66% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.65% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8249` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5165` |
| 29 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 32 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5213` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6946` |
| 36 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 37 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 39 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 41 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5166` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 44 | 0.38% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1707` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5184` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 13603` |
| 47 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 48 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7336` |
| 50 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.appendItem(ItemEntity, int) line: 6752` |
| 54 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 55 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5332` |
| 57 | 0.26% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6873` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8229` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5735` |
| 5 | 2.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 9 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13116` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 13 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.03% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 15 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.isInFluid(TagKey) line: 186` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 6659` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 19 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 20 | 0.87% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 22 | 0.77% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.72% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 9262` |
| 24 | 0.72% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.67% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 6664` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.56% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

