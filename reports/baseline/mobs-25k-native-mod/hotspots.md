# JVM hotspots (JFR ExecutionSample)

Samples: 3306

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12892` |
| 2 | 3.66% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.42% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 6 | 1.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.67% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 13 | 0.64% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.54% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 18 | 0.54% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 20 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 22 | 0.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.45% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 26 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.39% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.39% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 30 | 0.36% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.36% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 32 | 0.36% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 33 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 34 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 35 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 37 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13134` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9820` |
| 40 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 47 | 0.27% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 48 | 0.27% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 51 | 0.27% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 52 | 0.27% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 53 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12911` |
| 54 | 0.24% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1864` |
| 55 | 0.24% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 56 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.21% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 58 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.21% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 647` |
| 60 | 0.21% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13134` |
| 2 | 3.87% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11066` |
| 3 | 3.57% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11133` |
| 4 | 3.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.09% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.03% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 13 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6827` |
| 14 | 0.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 15 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.73% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 17 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.60% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.57% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8250` |
| 23 | 0.51% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.48% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 29 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.42% | `java.util.HashMap.containsKey(Object) line: 603` |

GC pauses: 0, total 0 ms

