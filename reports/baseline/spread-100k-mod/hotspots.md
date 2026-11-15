# JVM hotspots (JFR ExecutionSample)

Samples: 7875

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7963` |
| 4 | 4.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 5 | 3.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 6 | 3.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.95% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5388` |
| 9 | 2.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7717` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7773` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13663` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12991` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13681` |
| 17 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.94% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.76% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 22 | 0.74% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 23 | 0.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.58% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 26 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 27 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.47% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 29 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 30 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 31 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5390` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 34 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 37 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 38 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.24% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 40 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 42 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 45 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 47 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 48 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 49 | 0.20% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13872` |
| 51 | 0.20% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 52 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 54 | 0.19% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.19% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13918` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13635` |
| 58 | 0.18% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2503` |
| 60 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 8.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 4 | 5.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 5 | 4.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5933` |
| 8 | 1.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 9 | 1.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8754` |
| 11 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 12 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 0.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13008` |
| 14 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11633` |
| 17 | 0.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.66% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7703` |
| 21 | 0.58% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11586` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12999` |
| 24 | 0.50% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7699` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5108` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11566` |
| 28 | 0.42% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8750` |

GC pauses: 0, total 0 ms

