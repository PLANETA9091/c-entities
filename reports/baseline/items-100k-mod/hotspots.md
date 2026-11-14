# JVM hotspots (JFR ExecutionSample)

Samples: 5120

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 2 | 5.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 4 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7434` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.rmDead(int, long) line: 2557` |
| 10 | 2.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2486` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 14 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 16 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 17 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2568` |
| 19 | 0.90% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12782` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5595` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7416` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2482` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.setIterating(Object, boolean) line: 12611` |
| 31 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2481` |
| 34 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2491` |
| 36 | 0.41% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2571` |
| 37 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.35% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7272` |
| 40 | 0.27% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 41 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 45 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2569` |
| 49 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 50 | 0.21% | `com.google.common.collect.ImmutableList.forEach(Consumer) line: 420` |
| 51 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7273` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2573` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7296` |
| 55 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 56 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7632` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 59 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 5.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5836` |
| 4 | 3.85% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 3.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.36% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2457` |
| 8 | 3.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5285` |
| 9 | 2.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 11 | 2.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7264` |
| 12 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5329` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 14 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7420` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13011` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13019` |
| 19 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 21 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.66% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13133` |
| 24 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.probeRemove(Entity) line: 1809` |
| 28 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 29 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

