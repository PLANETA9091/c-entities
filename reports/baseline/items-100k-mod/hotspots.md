# JVM hotspots (JFR ExecutionSample)

Samples: 4264

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 2 | 8.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 3 | 6.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 5 | 4.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 6 | 4.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.24% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 3.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 9 | 2.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 2.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 2.13% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2826` |
| 12 | 1.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 14 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 18 | 0.73% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.70% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7477` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7787` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7479` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7797` |
| 28 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 774` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7794` |
| 33 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4223` |
| 37 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3264` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4177` |
| 41 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 44 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 47 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 49 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7791` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4247` |
| 53 | 0.14% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4126` |
| 55 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4128` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2870` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1694` |
| 59 | 0.14% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 4 | 6.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 5 | 5.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2786` |
| 6 | 3.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 7 | 3.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2668` |
| 8 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2785` |
| 11 | 2.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 12 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 16 | 1.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7790` |
| 17 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7782` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2766` |
| 21 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 23 | 0.52% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 29 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

