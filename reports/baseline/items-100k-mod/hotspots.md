# JVM hotspots (JFR ExecutionSample)

Samples: 3820

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9528` |
| 2 | 6.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 3 | 5.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 2.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15237` |
| 9 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.13% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 11 | 1.07% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16709` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9274` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9709` |
| 21 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16711` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3235` |
| 24 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8963` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6779` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4088` |
| 29 | 0.34% | `java.util.ArrayList$Itr.next() line: 1058` |
| 30 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10842` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16723` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6769` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 35 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 36 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9246` |
| 40 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 41 | 0.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 43 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.26% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6273` |
| 47 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16710` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 17038` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6318` |
| 51 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6392` |
| 52 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9127` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3245` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9712` |
| 57 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 58 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 3 | 7.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.77% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6505` |
| 7 | 2.04% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 8 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9246` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6273` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15229` |
| 13 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9250` |
| 15 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 0.58% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 18 | 0.55% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3725` |
| 20 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9380` |
| 22 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16922` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 30 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |

GC pauses: 0, total 0 ms

