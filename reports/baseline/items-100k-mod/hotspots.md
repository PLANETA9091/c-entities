# JVM hotspots (JFR ExecutionSample)

Samples: 6018

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 2 | 7.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 3 | 5.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 7 | 2.91% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |
| 16 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.78% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13414` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2844` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 24 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8080` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 27 | 0.35% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2849` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14405` |
| 31 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14058` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 41 | 0.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 71` |
| 42 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2843` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7865` |
| 45 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 47 | 0.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14322` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6049` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5738` |
| 52 | 0.15% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 53 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 55 | 0.13% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 56 | 0.13% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 57 | 0.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.13% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9149` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 4 | 5.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 5 | 4.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.36% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14335` |
| 9 | 1.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 12 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13406` |
| 15 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 17 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.33% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 21 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8174` |
| 22 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.27% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2849` |
| 27 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14297` |

GC pauses: 0, total 0 ms

