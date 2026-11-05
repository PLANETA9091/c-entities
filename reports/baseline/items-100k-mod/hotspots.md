# JVM hotspots (JFR ExecutionSample)

Samples: 5225

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.58% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 12 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 14 | 0.96% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |
| 16 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5988` |
| 24 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3091` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 28 | 0.34% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5859` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5906` |
| 32 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 33 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 36 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5768` |
| 41 | 0.27% | `java.lang.Double.isFinite(double) line: 1023` |
| 42 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4330` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5903` |
| 46 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3097` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 51 | 0.21% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 52 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 55 | 0.19% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5998` |
| 57 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 58 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 60 | 0.17% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 2 | 14.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 5 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 6 | 3.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 7 | 2.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 8 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 10 | 1.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 12 | 1.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 15 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.65% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 17 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 18 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4338` |
| 23 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.27% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 28 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3096` |

GC pauses: 0, total 0 ms

