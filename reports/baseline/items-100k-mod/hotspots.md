# JVM hotspots (JFR ExecutionSample)

Samples: 4781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 2 | 5.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9255` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 5 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6632` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15189` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9016` |
| 10 | 1.69% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14963` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 0.71% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6641` |
| 18 | 0.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10471` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 23 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10568` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3172` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6310` |
| 28 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 30 | 0.36% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.36% | `java.util.ArrayList$Itr.next() line: 1058` |
| 33 | 0.36% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 34 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8990` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16110` |
| 37 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6207` |
| 39 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10551` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8895` |
| 45 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 47 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 49 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 51 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10483` |
| 56 | 0.23% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16108` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10550` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6301` |
| 60 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 4 | 5.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.93% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6367` |
| 6 | 2.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 1.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16186` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8990` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6182` |
| 12 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14955` |
| 17 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15758` |
| 20 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 21 | 0.56% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1054` |
| 25 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9107` |
| 26 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.36% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |

GC pauses: 0, total 0 ms

