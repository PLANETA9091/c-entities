# JVM hotspots (JFR ExecutionSample)

Samples: 4180

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7983` |
| 2 | 6.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5458` |
| 6 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 7 | 3.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7791` |
| 8 | 2.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.87% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13566` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7653` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7787` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5460` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13988` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13058` |
| 18 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 20 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 23 | 0.50% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2574` |
| 25 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2578` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14049` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8804` |
| 30 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 35 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2583` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5330` |
| 40 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.19% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 42 | 0.19% | `java.util.ArrayList$Itr.next() line: 1058` |
| 43 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 45 | 0.19% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7581` |
| 47 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 48 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 51 | 0.17% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 52 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 54 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13705` |
| 58 | 0.14% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7548` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8804` |
| 3 | 5.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5342` |
| 4 | 5.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6003` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13979` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7773` |
| 9 | 1.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 12 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13050` |
| 16 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7791` |
| 19 | 0.45% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 20 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 21 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 24 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.29% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2583` |
| 26 | 0.26% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 27 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 28 | 0.24% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.24% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |

GC pauses: 0, total 0 ms

