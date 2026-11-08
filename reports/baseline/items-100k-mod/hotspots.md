# JVM hotspots (JFR ExecutionSample)

Samples: 3894

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4627` |
| 3 | 5.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4759` |
| 4 | 4.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 5 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6266` |
| 7 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.57% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 9 | 2.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4707` |
| 10 | 2.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7112` |
| 12 | 1.64% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 14 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6410` |
| 16 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6339` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10815` |
| 19 | 0.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3396` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6443` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10827` |
| 26 | 0.36% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 27 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6289` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4702` |
| 31 | 0.31% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.28% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 33 | 0.28% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10831` |
| 37 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4620` |
| 42 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 46 | 0.21% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10866` |
| 48 | 0.21% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3347` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 422` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 406` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 53 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.setHeight(int, int, int) line: 139` |
| 54 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3354` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6257` |
| 57 | 0.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 58 | 0.15% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4703` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6396` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6257` |
| 2 | 7.99% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7112` |
| 4 | 5.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4660` |
| 5 | 4.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 4.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 3.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4524` |
| 9 | 3.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4658` |
| 10 | 2.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4628` |
| 11 | 2.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2633` |
| 12 | 2.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6396` |
| 14 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4627` |
| 15 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11060` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11047` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4979` |
| 19 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.54% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.51% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6414` |
| 26 | 0.36% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 455` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.28% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

