# JVM hotspots (JFR ExecutionSample)

Samples: 4083

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 3 | 4.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 4 | 3.09% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 5 | 2.79% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 7 | 2.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 8 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3493` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 13 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 17 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2218` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2120` |
| 20 | 0.54% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 0.54% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3530` |
| 23 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2109` |
| 25 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.37% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 27 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.32% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3499` |
| 30 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 35 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 36 | 0.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4046` |
| 38 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.24% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 40 | 0.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 41 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 42 | 0.22% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 43 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4037` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4027` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 50 | 0.20% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 52 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.17% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |
| 54 | 0.17% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 56 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6349` |
| 58 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 59 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 60 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 2 | 8.43% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 4 | 5.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 3.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 2.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 8 | 2.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 2.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3530` |
| 11 | 2.16% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6532` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 14 | 1.05% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2141` |
| 18 | 0.56% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 19 | 0.51% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 20 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 24 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 28 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 206` |
| 29 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 30 | 0.29% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |

GC pauses: 0, total 0 ms

