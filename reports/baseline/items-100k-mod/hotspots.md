# JVM hotspots (JFR ExecutionSample)

Samples: 6043

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 6 | 3.14% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 8 | 2.17% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 21 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3687` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8739` |
| 25 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 28 | 0.30% | `java.util.HashMap.getNode(Object) line: 577` |
| 29 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3511` |
| 30 | 0.26% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 31 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4915` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5003` |
| 38 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.22% | `java.lang.Integer.valueOf(int) line: 1006` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4916` |
| 42 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5142` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5175` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8429` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.17% | `java.util.HashMap.getNode(Object) line: 576` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3504` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3731` |
| 57 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 58 | 0.15% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 59 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2652` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.09% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 4.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 5 | 4.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 7 | 3.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 8 | 2.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 9 | 2.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 10 | 2.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 11 | 1.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 16 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 21 | 0.55% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8742` |
| 23 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 675` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3608` |
| 30 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

