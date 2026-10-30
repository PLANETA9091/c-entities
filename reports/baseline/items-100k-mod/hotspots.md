# JVM hotspots (JFR ExecutionSample)

Samples: 6357

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 2 | 11.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 6.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.37% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 7 | 2.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 11 | 0.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |
| 13 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 16 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4790` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5519` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 23 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4609` |
| 25 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3380` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4608` |
| 29 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 30 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 31 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8432` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8122` |
| 34 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 36 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4835` |
| 37 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 39 | 0.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 40 | 0.19% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4868` |
| 42 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4796` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3222` |
| 44 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 45 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 46 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2429` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4789` |
| 49 | 0.17% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 50 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 52 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.16% | `java.lang.Integer.valueOf(int) line: 1006` |
| 58 | 0.16% | `java.util.HashMap.getNode(Object) line: 577` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4738` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4739` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.18% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 5 | 4.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3340` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5519` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2435` |
| 9 | 1.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3222` |
| 10 | 1.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1910` |
| 11 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2081` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2452` |
| 17 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3339` |
| 19 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 22 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 24 | 0.35% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.31% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.30% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 27 | 0.28% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

