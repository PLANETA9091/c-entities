# JVM hotspots (JFR ExecutionSample)

Samples: 6156

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 10.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 4 | 4.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 2.86% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 8 | 2.40% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 10 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 16 | 1.14% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3687` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 18 | 0.75% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8739` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2231` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3511` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4916` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3600` |
| 30 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5142` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8429` |
| 33 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 35 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 40 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5103` |
| 43 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.21% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 45 | 0.21% | `java.util.HashMap.getNode(Object) line: 577` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2673` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5046` |
| 50 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2652` |
| 53 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8746` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3619` |
| 56 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2651` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5096` |
| 59 | 0.16% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 60 | 0.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 4 | 5.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |
| 5 | 4.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 6 | 2.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 2.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 9 | 2.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 10 | 2.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 11 | 1.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 15 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8742` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 19 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 726` |
| 21 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8734` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3608` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.47% | `java.util.HashMap.get(Object) line: 565` |
| 27 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |

GC pauses: 0, total 0 ms

