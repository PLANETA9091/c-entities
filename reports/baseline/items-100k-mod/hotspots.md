# JVM hotspots (JFR ExecutionSample)

Samples: 4234

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.95% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 2 | 6.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2107` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 5 | 3.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 6 | 2.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 10 | 1.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3491` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4044` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 19 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3528` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2216` |
| 22 | 0.47% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 23 | 0.47% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2042` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.38% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2036` |
| 31 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 33 | 0.31% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2064` |
| 35 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 39 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 41 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 43 | 0.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3993` |
| 44 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2016` |
| 46 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3497` |
| 49 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 50 | 0.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 51 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2118` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 54 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 56 | 0.19% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4035` |
| 58 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 59 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 60 | 0.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 2 | 11.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 3 | 5.79% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 4 | 5.10% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 5 | 4.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 8 | 2.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3528` |
| 9 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4035` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 15 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.61% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 17 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 18 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2139` |
| 20 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6457` |
| 26 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2033` |
| 30 | 0.33% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

