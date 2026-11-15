# JVM hotspots (JFR ExecutionSample)

Samples: 2796

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 2 | 5.97% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 1.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 7 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12972` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 18 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.54% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 21 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 22 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 27 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 28 | 0.39% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 29 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 30 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 34 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.36% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.36% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 39 | 0.32% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 40 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 41 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 43 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 46 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 47 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 48 | 0.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11156` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 54 | 0.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 55 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 56 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.21% | `java.util.ArrayList.isEmpty() line: 263` |
| 58 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.21% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 60 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 2 | 5.69% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 4 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.47% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6844` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8311` |
| 14 | 0.97% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 17 | 0.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.72% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11147` |
| 21 | 0.64% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 25 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.39% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.39% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 28 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 29 | 0.39% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |

GC pauses: 0, total 0 ms

