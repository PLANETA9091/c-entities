# JVM hotspots (JFR ExecutionSample)

Samples: 5722

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 2 | 8.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 5.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 4 | 5.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 6 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.46% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 10 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 18 | 0.66% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5419` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8769` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3959` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3872` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8767` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3891` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5232` |
| 31 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.33% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 33 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.31% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 36 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 37 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 40 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5491` |
| 42 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3885` |
| 45 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5418` |
| 47 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 49 | 0.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3776` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3783` |
| 53 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 54 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4003` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3912` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 58 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 59 | 0.17% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 60 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.00% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 3 | 6.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 5 | 3.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 6 | 3.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 8 | 2.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3912` |
| 9 | 2.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3781` |
| 10 | 2.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 11 | 2.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 13 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3910` |
| 18 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9080` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3880` |
| 21 | 0.56% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 23 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1166` |
| 25 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |

GC pauses: 0, total 0 ms

