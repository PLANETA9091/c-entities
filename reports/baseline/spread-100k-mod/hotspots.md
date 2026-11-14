# JVM hotspots (JFR ExecutionSample)

Samples: 15817

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 4 | 3.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.67% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 12 | 1.33% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 14 | 1.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.04% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.96% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.89% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.89% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2438` |
| 21 | 0.78% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 22 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 23 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.58% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5817` |
| 29 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.49% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 31 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 33 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.getFluidHeight(TagKey) line: 176` |
| 34 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5165` |
| 35 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 39 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 40 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5880` |
| 41 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6946` |
| 42 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 13603` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2442` |
| 45 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 46 | 0.30% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1707` |
| 47 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9815` |
| 49 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 50 | 0.29% | `dev.planeta9091.centities.CeHooks.appendItem(ItemEntity, int) line: 6752` |
| 51 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7200` |
| 54 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 56 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5318` |
| 57 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7366` |
| 58 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 60 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8229` |
| 6 | 3.33% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5735` |
| 7 | 3.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.59% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7344` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 12 | 1.14% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 13 | 1.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8249` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 0.89% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11061` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11128` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2928` |
| 21 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 24 | 0.62% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.60% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 6664` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 27 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.54% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 30 | 0.54% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |

GC pauses: 0, total 0 ms

