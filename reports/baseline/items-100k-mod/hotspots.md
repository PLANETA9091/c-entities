# JVM hotspots (JFR ExecutionSample)

Samples: 6344

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 2 | 10.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4382` |
| 3 | 8.07% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 7 | 3.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3077` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8025` |
| 12 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2209` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4388` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 17 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2969` |
| 20 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1868` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3030` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5102` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7705` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 27 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7707` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4427` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2193` |
| 31 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.27% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 36 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 40 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 41 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 42 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2187` |
| 44 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 45 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4243` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4330` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4329` |
| 51 | 0.17% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8019` |
| 53 | 0.16% | `java.lang.Double.isFinite(double) line: 1023` |
| 54 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 906` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4430` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4331` |
| 59 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.14% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.12% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 8.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8060` |
| 5 | 5.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 6 | 4.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 7 | 3.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2990` |
| 8 | 2.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 9 | 1.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5102` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2872` |
| 12 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2194` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2989` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8018` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 548` |
| 20 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2969` |
| 25 | 0.28% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.27% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 27 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8010` |
| 29 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8022` |
| 30 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

