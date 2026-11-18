# JVM hotspots (JFR ExecutionSample)

Samples: 3507

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.14% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 3 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 4 | 2.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 5 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9860` |
| 7 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.00% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15013` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 17 | 0.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15031` |
| 20 | 0.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.60% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2473` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.51% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.43% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 30 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 31 | 0.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2670` |
| 33 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 34 | 0.37% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.37% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 37 | 0.34% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 38 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12835` |
| 39 | 0.31% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 40 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 42 | 0.31% | `java.util.Objects.equals(Object, Object) line: 62` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11539` |
| 44 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 47 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.29% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.26% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 53 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 55 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 56 | 0.23% | `java.lang.Class.cast(Object) line: 3491` |
| 57 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12814` |
| 59 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12852` |
| 2 | 4.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 3 | 3.82% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12785` |
| 4 | 3.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7845` |
| 10 | 1.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15016` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14218` |
| 14 | 0.97% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9875` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12805` |
| 19 | 0.63% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.54% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15019` |
| 27 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15021` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2608` |

GC pauses: 0, total 0 ms

