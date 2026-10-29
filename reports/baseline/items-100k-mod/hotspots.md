# JVM hotspots (JFR ExecutionSample)

Samples: 5177

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8237` |
| 2 | 7.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4555` |
| 3 | 7.44% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8184` |
| 6 | 4.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3250` |
| 8 | 2.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3142` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5273` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4561` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5275` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3203` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2317` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8198` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8188` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4592` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7878` |
| 25 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1968` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4600` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3135` |
| 29 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 30 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7880` |
| 33 | 0.33% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 41 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8195` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3047` |
| 44 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4505` |
| 47 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.17% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 50 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2320` |
| 54 | 0.15% | `java.lang.Double.isFinite(double) line: 1023` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3247` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8192` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4416` |
| 59 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 11.38% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8233` |
| 5 | 5.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4592` |
| 6 | 4.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3163` |
| 7 | 3.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5275` |
| 8 | 2.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8225` |
| 9 | 2.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3045` |
| 10 | 2.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 2.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1809` |
| 13 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8191` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3142` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3162` |
| 17 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2321` |
| 21 | 0.66% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 563` |
| 22 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8183` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 539` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3143` |
| 28 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8184` |
| 30 | 0.27% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

