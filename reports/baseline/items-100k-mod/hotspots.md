# JVM hotspots (JFR ExecutionSample)

Samples: 5214

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 2 | 6.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 3.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 5 | 3.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 6 | 3.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10091` |
| 8 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6050` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10095` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5953` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 16 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 18 | 0.92% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10137` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6752` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4416` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6036` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 27 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5850` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 33 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 36 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.27% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5758` |
| 39 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4418` |
| 41 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 46 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.23% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5894` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5892` |
| 50 | 0.23% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 51 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 52 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6743` |
| 54 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 55 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 56 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 59 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 2 | 12.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.27% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 3.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4369` |
| 5 | 3.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 6 | 3.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 7 | 3.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4233` |
| 8 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6752` |
| 10 | 1.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4367` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6036` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 17 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.46% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 20 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.35% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 24 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 27 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6054` |
| 29 | 0.23% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4236` |

GC pauses: 0, total 0 ms

