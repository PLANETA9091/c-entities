# JVM hotspots (JFR ExecutionSample)

Samples: 4503

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 2 | 4.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 3 | 4.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 5 | 4.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 7 | 2.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 9 | 2.55% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 10 | 2.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 11 | 1.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13414` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 22 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2844` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 27 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 29 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5738` |
| 31 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14405` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8080` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2849` |
| 35 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5764` |
| 37 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 39 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 40 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 44 | 0.20% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3264` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7897` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5647` |
| 47 | 0.20% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 50 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14100` |
| 53 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2843` |
| 56 | 0.13% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 57 | 0.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 226` |
| 58 | 0.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7898` |
| 60 | 0.13% | `net.minecraft.world.level.ChunkPos.z() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.47% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 3 | 5.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 6 | 2.78% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14335` |
| 8 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 9 | 1.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 12 | 1.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13406` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 17 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.71% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 19 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.51% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 21 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.40% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.27% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 30 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8174` |

GC pauses: 0, total 0 ms

