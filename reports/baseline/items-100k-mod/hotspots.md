# JVM hotspots (JFR ExecutionSample)

Samples: 3071

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 4.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 3 | 4.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 5 | 2.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 8 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 10 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 13 | 1.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 17 | 0.94% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2367` |
| 23 | 0.55% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2345` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 28 | 0.49% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.46% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 30 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2519` |
| 32 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 36 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 37 | 0.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2339` |
| 39 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2319` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 43 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2321` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2368` |
| 48 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 49 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 51 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3875` |
| 52 | 0.20% | `java.util.ArrayList.grow(int) line: 238` |
| 53 | 0.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.20% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7392` |
| 57 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 58 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 59 | 0.16% | `jdk.internal.misc.Unsafe.allocateInstance(Class)` |
| 60 | 0.16% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3290` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 2 | 8.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 3 | 4.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 6 | 2.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2336` |
| 9 | 2.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 12 | 1.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7416` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 15 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7408` |
| 16 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 17 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.85% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 19 | 0.72% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.72% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2442` |
| 26 | 0.46% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 27 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 28 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2404` |

GC pauses: 0, total 0 ms

