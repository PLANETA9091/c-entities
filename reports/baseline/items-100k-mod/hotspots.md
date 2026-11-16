# JVM hotspots (JFR ExecutionSample)

Samples: 3548

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 2 | 5.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 5 | 2.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 6 | 2.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 9 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 13 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7718` |
| 16 | 0.76% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2631` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13162` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 22 | 0.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 23 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 26 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2635` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 31 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 32 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7828` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5524` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7528` |
| 35 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 38 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5486` |
| 40 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7856` |
| 41 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2640` |
| 44 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 45 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7646` |
| 47 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 48 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7185` |
| 53 | 0.20% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 54 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5311` |
| 56 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 57 | 0.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 59 | 0.17% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14153` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 3 | 5.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5407` |
| 4 | 4.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 7 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 8 | 1.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14083` |
| 12 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 0.82% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13154` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7838` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 18 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.48% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7922` |
| 22 | 0.48% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7856` |
| 24 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.42% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |

GC pauses: 0, total 0 ms

