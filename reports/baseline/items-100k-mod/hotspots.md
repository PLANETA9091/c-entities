# JVM hotspots (JFR ExecutionSample)

Samples: 4098

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7189` |
| 2 | 8.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5297` |
| 3 | 4.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7096` |
| 5 | 3.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 8 | 2.12% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7952` |
| 12 | 1.54% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6714` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7092` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12424` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7074` |
| 17 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2442` |
| 23 | 0.41% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2438` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5047` |
| 26 | 0.39% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 27 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 28 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.27% | `java.util.ArrayList$Itr.next() line: 1058` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7068` |
| 31 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 34 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6931` |
| 37 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 38 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 40 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 43 | 0.17% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 44 | 0.17% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 45 | 0.17% | `net.minecraft.nbt.CompoundTag.<init>() line: 195` |
| 46 | 0.17% | `java.lang.Double.isFinite(double) line: 1023` |
| 47 | 0.15% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 48 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 49 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5060` |
| 53 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.12% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 56 | 0.12% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 57 | 0.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 58 | 0.10% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.10% | `java.util.ArrayList.grow(int) line: 238` |
| 60 | 0.10% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5055` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5026` |
| 4 | 3.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5050` |
| 6 | 3.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 3.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.12% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5533` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7078` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7074` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7952` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12656` |
| 14 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.81% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 16 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7096` |
| 20 | 0.32% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 26 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 891` |
| 28 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 29 | 0.22% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.22% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

