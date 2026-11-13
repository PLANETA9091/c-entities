# JVM hotspots (JFR ExecutionSample)

Samples: 3160

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 2 | 3.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 6 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 10 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11881` |
| 13 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.76% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 17 | 0.73% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 22 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.51% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.51% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.44% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 30 | 0.44% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 31 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 32 | 0.41% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1895` |
| 33 | 0.38% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 36 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11899` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 39 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 42 | 0.35% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1894` |
| 43 | 0.35% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2022` |
| 44 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 46 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 47 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8956` |
| 49 | 0.25% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 51 | 0.25% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 52 | 0.25% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 53 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 54 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 55 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 56 | 0.22% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 57 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.22% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 59 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.22% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 2 | 3.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10202` |
| 3 | 2.97% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10269` |
| 4 | 2.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 7 | 2.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7386` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.73% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10222` |
| 19 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6207` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11887` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10230` |
| 27 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.44% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 30 | 0.38% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

