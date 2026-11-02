# JVM hotspots (JFR ExecutionSample)

Samples: 4752

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 3 | 5.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4033` |
| 4 | 4.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9354` |
| 6 | 3.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3906` |
| 8 | 2.17% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.77% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3986` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2712` |
| 14 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5459` |
| 16 | 1.22% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2750` |
| 18 | 0.93% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9349` |
| 20 | 0.82% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5563` |
| 22 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9348` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5530` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2724` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.29% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3899` |
| 32 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 33 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.23% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5515` |
| 37 | 0.23% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5268` |
| 39 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5269` |
| 41 | 0.21% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 42 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.21% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 45 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5511` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3810` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5513` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2703` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5359` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6223` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9379` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3983` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5464` |
| 58 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 59 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 60 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9353` |
| 3 | 6.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9456` |
| 4 | 5.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3939` |
| 5 | 4.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 4.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 7 | 3.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 9 | 3.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 2.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9446` |
| 11 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3808` |
| 12 | 2.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 2.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3937` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2725` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5515` |
| 16 | 1.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3906` |
| 19 | 0.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9455` |
| 20 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 22 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 23 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.27% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 30 | 0.25% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |

GC pauses: 0, total 0 ms

