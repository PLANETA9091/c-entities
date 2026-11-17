# JVM hotspots (JFR ExecutionSample)

Samples: 3115

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8660` |
| 3 | 4.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 4 | 3.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7701` |
| 6 | 2.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7688` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7703` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 10 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 12 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8446` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13781` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14782` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14796` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5994` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5990` |
| 19 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5948` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7715` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 984` |
| 23 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.58% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8424` |
| 27 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 29 | 0.45% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 30 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.42% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.39% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5996` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8442` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2964` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2959` |
| 41 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 45 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5957` |
| 47 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2955` |
| 49 | 0.26% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 50 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 52 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15000` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14792` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14615` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5895` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8332` |
| 58 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7793` |
| 2 | 8.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 5 | 3.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5896` |
| 8 | 2.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 11 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13773` |
| 15 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 0.74% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8424` |
| 18 | 0.64% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7724` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7718` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 986` |
| 22 | 0.58% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5925` |
| 24 | 0.51% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8512` |
| 26 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.42% | `java.util.ArrayList.grow(int) line: 238` |
| 28 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks$RefField.getInt(Object) line: 7332` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7723` |

GC pauses: 0, total 0 ms

