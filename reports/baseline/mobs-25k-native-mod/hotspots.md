# JVM hotspots (JFR ExecutionSample)

Samples: 3308

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 2 | 3.26% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 5 | 2.39% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 6 | 2.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 11 | 1.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.85% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 15 | 0.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.57% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 18 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 20 | 0.54% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.48% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 23 | 0.48% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 24 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12042` |
| 26 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 27 | 0.39% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 28 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 31 | 0.36% | `java.util.ArrayList$Itr.next() line: 1058` |
| 32 | 0.36% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 33 | 0.36% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 34 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 40 | 0.30% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10746` |
| 42 | 0.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 43 | 0.30% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2314` |
| 45 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 46 | 0.27% | `java.lang.Math.unsignedMultiplyHigh(long, long) line: 1433` |
| 47 | 0.24% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 48 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 49 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2505` |
| 51 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12020` |
| 54 | 0.18% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 55 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 56 | 0.18% | `com.google.common.collect.RegularImmutableMap$KeySet.get(int) line: 326` |
| 57 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 58 | 0.18% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 59 | 0.18% | `com.google.common.collect.ImmutableMap.keySet() line: 71` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 2 | 3.72% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 4 | 3.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 8 | 1.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7504` |
| 12 | 1.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9176` |
| 16 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 17 | 0.67% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 19 | 0.63% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.57% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.48% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 25 | 0.42% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 27 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12020` |
| 29 | 0.39% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |

GC pauses: 0, total 0 ms

