# JVM hotspots (JFR ExecutionSample)

Samples: 5756

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.12% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 3 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6665` |
| 5 | 3.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.81% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11883` |
| 8 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 9 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4901` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.29% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6300` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 0.71% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 18 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7411` |
| 20 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6622` |
| 21 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6608` |
| 22 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3475` |
| 23 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.24% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 28 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6626` |
| 29 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 30 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 32 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 34 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 38 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 40 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6469` |
| 41 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 42 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 43 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 44 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.16% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 46 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6551` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6606` |
| 49 | 0.14% | `java.util.HashMap.getNode(Object) line: 579` |
| 50 | 0.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.14% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 647` |
| 52 | 0.14% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 53 | 0.14% | `java.util.Collection.stream() line: 748` |
| 54 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 55 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.14% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 57 | 0.14% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.14% | `java.util.ArrayList$Itr.next() line: 1058` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6745` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6452` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6469` |
| 2 | 8.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 7.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6551` |
| 5 | 4.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 4.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.42% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5137` |
| 8 | 2.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12115` |
| 9 | 2.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4641` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4669` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6608` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2698` |
| 16 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.50% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7411` |
| 22 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4664` |
| 24 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.24% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 26 | 0.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 30 | 0.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |

GC pauses: 0, total 0 ms

