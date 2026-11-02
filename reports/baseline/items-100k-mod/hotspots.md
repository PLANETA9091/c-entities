# JVM hotspots (JFR ExecutionSample)

Samples: 3408

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 2 | 8.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 3 | 4.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 5 | 3.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 6 | 2.90% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 8 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9625` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 12 | 1.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5669` |
| 14 | 1.73% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 1.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 18 | 1.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9632` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5636` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9588` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9585` |
| 28 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9593` |
| 31 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 32 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2751` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4025` |
| 36 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9808` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 40 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 41 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4032` |
| 43 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5620` |
| 48 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 51 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2721` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2729` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5495` |
| 56 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3943` |
| 58 | 0.18% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 59 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 3 | 5.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 4 | 4.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 5 | 4.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 6 | 4.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 7 | 4.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 8 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 9 | 2.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3951` |
| 13 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.20% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 17 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 19 | 0.59% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.53% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 28 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5640` |
| 29 | 0.23% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

