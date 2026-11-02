# JVM hotspots (JFR ExecutionSample)

Samples: 3254

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5494` |
| 2 | 7.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4056` |
| 3 | 4.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3929` |
| 4 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6272` |
| 7 | 2.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5499` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4009` |
| 9 | 2.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 2.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2724` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 13 | 1.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5570` |
| 15 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9543` |
| 17 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2762` |
| 18 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5603` |
| 19 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6262` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9506` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4004` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5310` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9503` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 30 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 32 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.31% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9511` |
| 35 | 0.31% | `java.lang.Double.isFinite(double) line: 1023` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4005` |
| 37 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5399` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2714` |
| 41 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |
| 43 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2715` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5311` |
| 47 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.22% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 49 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5400` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3833` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5493` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5498` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 59 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 60 | 0.15% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3962` |
| 3 | 5.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6272` |
| 4 | 4.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2106` |
| 5 | 4.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3826` |
| 6 | 3.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3960` |
| 7 | 3.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3929` |
| 9 | 2.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9680` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3930` |
| 12 | 1.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2737` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9679` |
| 17 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.55% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 23 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5574` |
| 26 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 29 | 0.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |

GC pauses: 0, total 0 ms

