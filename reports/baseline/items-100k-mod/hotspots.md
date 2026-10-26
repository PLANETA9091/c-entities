# JVM hotspots (JFR ExecutionSample)

Samples: 4618

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 2 | 10.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 7.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 5.24% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 5 | 2.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 7 | 2.06% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 10 | 1.69% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2219` |
| 16 | 0.63% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 19 | 0.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 21 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4047` |
| 22 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 23 | 0.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 24 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 26 | 0.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 27 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 29 | 0.28% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3539` |
| 31 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 32 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 33 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 34 | 0.24% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 35 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 38 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 41 | 0.22% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 42 | 0.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 43 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 44 | 0.17% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 45 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 48 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 49 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 50 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 51 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6686` |
| 54 | 0.15% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 56 | 0.13% | `java.util.Objects.equals(Object, Object) line: 62` |
| 57 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 58 | 0.13% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 651` |
| 59 | 0.13% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 60 | 0.13% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.26% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 9.85% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 3 | 7.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 4.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 6 | 3.88% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 8 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6869` |
| 11 | 1.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 15 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2142` |
| 17 | 0.61% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 20 | 0.54% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.37% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 22 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.37% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 24 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 25 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2104` |
| 27 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 29 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 206` |
| 30 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

