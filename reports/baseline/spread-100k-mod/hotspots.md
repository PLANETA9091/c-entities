# JVM hotspots (JFR ExecutionSample)

Samples: 9796

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 4.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 3 | 4.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 4 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 9 | 2.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 11 | 1.66% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 14 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.11% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13040` |
| 19 | 0.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.72% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 22 | 0.64% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 23 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 27 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7388` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9931` |
| 30 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.37% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 34 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 35 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.32% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 37 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 38 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 43 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5196` |
| 47 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 48 | 0.22% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 50 | 0.21% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 51 | 0.20% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 52 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8365` |
| 54 | 0.20% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 55 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 56 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2459` |
| 58 | 0.19% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 59 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 60 | 0.18% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 7 | 3.17% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 10 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 12 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 16 | 0.77% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7388` |
| 18 | 0.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 0.69% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 22 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8365` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9938` |
| 26 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 28 | 0.40% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

