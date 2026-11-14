# JVM hotspots (JFR ExecutionSample)

Samples: 3825

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 2 | 7.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 3 | 7.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 4 | 4.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |
| 6 | 3.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 8 | 2.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 10 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.49% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7367` |
| 13 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12744` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.63% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 21 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.52% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 23 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7371` |
| 25 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.26% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 29 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 30 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 32 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7349` |
| 33 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 35 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 38 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 39 | 0.18% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 40 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5165` |
| 41 | 0.18% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 42 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5166` |
| 43 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7582` |
| 44 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.16% | `java.lang.Class.cast(Object) line: 3491` |
| 47 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2442` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7206` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7343` |
| 50 | 0.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 51 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 52 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 53 | 0.13% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 54 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5046` |
| 55 | 0.13% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 56 | 0.10% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 57 | 0.10% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 58 | 0.10% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 59 | 0.10% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 60 | 0.10% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.02% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 17.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 3 | 4.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 4 | 3.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 3.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7353` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 10 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7349` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13121` |
| 13 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 0.81% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 15 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 19 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 20 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 25 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7371` |
| 26 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7425` |
| 27 | 0.21% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.21% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 29 | 0.18% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |

GC pauses: 0, total 0 ms

