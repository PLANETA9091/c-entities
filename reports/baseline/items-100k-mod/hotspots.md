# JVM hotspots (JFR ExecutionSample)

Samples: 4389

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 2 | 5.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 3 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 6 | 2.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13209` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 17 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7765` |
| 20 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 23 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14200` |
| 25 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2644` |
| 27 | 0.39% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 28 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5844` |
| 30 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 31 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 32 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 36 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 38 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 40 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13853` |
| 43 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.18% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.18% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 46 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 48 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 49 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 50 | 0.16% | `java.util.BitSet.expandTo(int) line: 357` |
| 51 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 53 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 55 | 0.14% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.14% | `java.lang.String.hashCode() line: 2433` |
| 57 | 0.14% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |
| 59 | 0.14% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 60 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.94% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 3 | 4.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 4 | 4.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7885` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 8 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14130` |
| 9 | 1.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 13 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13201` |
| 15 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 0.68% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 19 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5581` |
| 22 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 23 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.34% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7969` |
| 26 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |

GC pauses: 0, total 0 ms

