# JVM hotspots (JFR ExecutionSample)

Samples: 4699

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 2 | 5.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 3 | 4.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 5 | 3.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 8 | 2.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12738` |
| 11 | 1.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7361` |
| 15 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 19 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7200` |
| 21 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7337` |
| 22 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 23 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 24 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5477` |
| 25 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5166` |
| 29 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7002` |
| 31 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 32 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 33 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |
| 35 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6862` |
| 36 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 42 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 44 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2438` |
| 45 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.15% | `java.lang.String.hashCode() line: 2433` |
| 47 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5192` |
| 48 | 0.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 50 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 51 | 0.13% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 52 | 0.13% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 53 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 54 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.13% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 56 | 0.13% | `java.lang.Class.cast(Object) line: 3491` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7199` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7365` |
| 59 | 0.13% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 60 | 0.13% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8228` |
| 3 | 4.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 4 | 4.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.64% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 7 | 3.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13115` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 11 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7343` |
| 15 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 17 | 0.40% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 18 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7419` |
| 21 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.28% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5214` |
| 27 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7365` |
| 30 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

