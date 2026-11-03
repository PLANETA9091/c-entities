# JVM hotspots (JFR ExecutionSample)

Samples: 4533

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.84% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 4 | 3.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 5 | 3.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9906` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9899` |
| 10 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9859` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5891` |
| 12 | 1.26% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5794` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.99% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9862` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4262` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5877` |
| 26 | 0.42% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2936` |
| 28 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5691` |
| 31 | 0.33% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.31% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 35 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4313` |
| 36 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5735` |
| 38 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2942` |
| 40 | 0.26% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2935` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10082` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5795` |
| 46 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 49 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 50 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.22% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9867` |
| 55 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |
| 59 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 3 | 6.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 5 | 4.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4215` |
| 6 | 3.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 7 | 3.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4079` |
| 8 | 2.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 2.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 10 | 2.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 12 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5877` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4213` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 17 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4183` |
| 19 | 0.51% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.51% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 22 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.42% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 25 | 0.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.31% | `jdk.internal.classfile.impl.SplitConstantPool.tryFindClassOrInterface(int, ClassDesc) line: 404` |
| 28 | 0.31% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 29 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4082` |

GC pauses: 0, total 0 ms

