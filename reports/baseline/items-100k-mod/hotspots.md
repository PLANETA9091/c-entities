# JVM hotspots (JFR ExecutionSample)

Samples: 4800

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 2 | 7.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 3 | 7.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 6 | 3.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 9 | 2.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 10 | 1.96% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |
| 14 | 1.33% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 1.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8787` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8479` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 28 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3636` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8477` |
| 31 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 34 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5190` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 37 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3718` |
| 41 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 42 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 43 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5051` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 46 | 0.21% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 47 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 49 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 51 | 0.17% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.17% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2681` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3655` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5156` |
| 57 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 58 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 59 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 2 | 8.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 7.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 5 | 4.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 6 | 3.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 8 | 2.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 10 | 2.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 11 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 12 | 1.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8790` |
| 13 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3644` |
| 18 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8782` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 25 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.42% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

