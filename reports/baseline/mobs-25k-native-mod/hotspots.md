# JVM hotspots (JFR ExecutionSample)

Samples: 4023

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 2 | 4.42% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 4 | 3.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 6 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.17% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 0.82% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 13 | 0.80% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14439` |
| 16 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.50% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 19 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14711` |
| 21 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.42% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 23 | 0.40% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 24 | 0.40% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 28 | 0.37% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14457` |
| 32 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.35% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 35 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 40 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.27% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 42 | 0.27% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 43 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 44 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 45 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 46 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 47 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 50 | 0.22% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2379` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2410` |
| 53 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.20% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 55 | 0.20% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 56 | 0.20% | `java.util.ArrayList$Itr.next() line: 1058` |
| 57 | 0.20% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 58 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 59 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14711` |
| 2 | 4.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12270` |
| 3 | 4.18% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12337` |
| 4 | 3.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 9 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14442` |
| 11 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7691` |
| 13 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.82% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13703` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.65% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.55% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.55% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9440` |
| 24 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.47% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 26 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.42% | `java.util.ArrayList.grow(int) line: 238` |
| 29 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

