# JVM hotspots (JFR ExecutionSample)

Samples: 5870

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 2 | 7.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 3 | 5.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.05% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7700` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13390` |
| 9 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12910` |
| 11 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13812` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 16 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7644` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7930` |
| 21 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7620` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 24 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7510` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13873` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 27 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.26% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 31 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5328` |
| 32 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5600` |
| 35 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 36 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 38 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7551` |
| 39 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13526` |
| 40 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.17% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 49 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 50 | 0.15% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.15% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 52 | 0.15% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 54 | 0.15% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |
| 55 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 57 | 0.14% | `java.util.Collection.stream() line: 748` |
| 58 | 0.14% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 59 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5870` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5210` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 6 | 3.27% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5871` |
| 7 | 3.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7630` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13803` |
| 9 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 12 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7626` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12902` |
| 17 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7714` |
| 20 | 0.27% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5337` |
| 23 | 0.27% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.22% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 27 | 0.22% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 28 | 0.20% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 29 | 0.20% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |

GC pauses: 0, total 0 ms

