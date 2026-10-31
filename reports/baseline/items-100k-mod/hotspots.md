# JVM hotspots (JFR ExecutionSample)

Samples: 5055

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 3 | 4.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.64% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 3.26% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 3.09% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 10 | 1.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3687` |
| 18 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.61% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8739` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 26 | 0.40% | `java.util.HashMap.getNode(Object) line: 577` |
| 27 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3511` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5142` |
| 31 | 0.34% | `java.lang.Integer.valueOf(int) line: 1006` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8429` |
| 33 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 38 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4916` |
| 41 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5044` |
| 45 | 0.20% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 46 | 0.20% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 47 | 0.20% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2673` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5103` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4915` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5047` |
| 52 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 53 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 57 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2652` |
| 60 | 0.16% | `java.util.HashMap.getNode(Object) line: 576` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 4.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 4 | 3.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 5 | 3.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 7 | 3.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 8 | 2.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 10 | 2.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 13 | 1.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 14 | 1.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 15 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 17 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8742` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 20 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.65% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8734` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3608` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 675` |
| 26 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |

GC pauses: 0, total 0 ms

