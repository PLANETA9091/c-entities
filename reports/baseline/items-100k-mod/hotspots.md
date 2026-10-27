# JVM hotspots (JFR ExecutionSample)

Samples: 3219

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 2 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 3 | 5.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2398` |
| 6 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 7 | 3.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 2.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3876` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2444` |
| 11 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.43% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7476` |
| 18 | 0.75% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.50% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2582` |
| 21 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2484` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1762` |
| 24 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2473` |
| 26 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 27 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3882` |
| 30 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 32 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 33 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4587` |
| 34 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 35 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.28% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1759` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3913` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3921` |
| 40 | 0.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 41 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2430` |
| 43 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7486` |
| 47 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7483` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2382` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7480` |
| 57 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 58 | 0.16% | `net.minecraft.world.phys.shapes.ArrayVoxelShape.<init>(DiscreteVoxelShape, DoubleList, DoubleList, DoubleList) line: 37` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 7.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4587` |
| 3 | 7.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7521` |
| 4 | 4.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2387` |
| 6 | 3.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7513` |
| 8 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2398` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3913` |
| 10 | 2.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 12 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7479` |
| 15 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2399` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1763` |
| 18 | 0.68% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7471` |
| 20 | 0.59% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2505` |
| 22 | 0.53% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 23 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.50% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2485` |
| 26 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2484` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2467` |

GC pauses: 0, total 0 ms

