# JVM hotspots (JFR ExecutionSample)

Samples: 10772

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 2 | 6.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 3 | 5.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 4 | 4.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 8 | 2.28% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.11% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4981` |
| 12 | 1.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 16 | 1.18% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.91% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 20 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.58% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 23 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5156` |
| 30 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 31 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5422` |
| 36 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12865` |
| 38 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12680` |
| 41 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7303` |
| 44 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7279` |
| 48 | 0.23% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 49 | 0.23% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9756` |
| 51 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 52 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.20% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 54 | 0.19% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8190` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10884` |
| 57 | 0.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11096` |
| 59 | 0.19% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.19% | `java.lang.invoke.Invokers.checkCustomized(MethodHandle) line: 626` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5676` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5167` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5094` |
| 9 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 10 | 1.18% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11069` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7289` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5118` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11002` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8190` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 19 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.58% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13070` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 23 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.50% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8186` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7285` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

GC pauses: 0, total 0 ms

