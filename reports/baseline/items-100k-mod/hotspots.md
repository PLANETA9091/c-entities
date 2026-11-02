# JVM hotspots (JFR ExecutionSample)

Samples: 4083

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 2 | 8.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 6.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 6 | 4.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.53% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 8 | 3.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 9 | 2.42% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3949` |
| 11 | 2.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 12 | 2.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5409` |
| 15 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9067` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 23 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5448` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8759` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 27 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8757` |
| 29 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3944` |
| 34 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 38 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5481` |
| 41 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5308` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3951` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9074` |
| 46 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 48 | 0.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 50 | 0.17% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 51 | 0.15% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 52 | 0.15% | `net.minecraft.world.phys.shapes.ArrayVoxelShape.<init>(DiscreteVoxelShape, DoubleList, DoubleList, DoubleList) line: 37` |
| 53 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 54 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 58 | 0.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2682` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5351` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5440` |
| 4 | 5.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9112` |
| 5 | 5.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3902` |
| 6 | 3.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 7 | 3.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 8 | 3.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3771` |
| 9 | 3.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3900` |
| 10 | 3.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 2.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 13 | 2.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9070` |
| 15 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3870` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9062` |
| 21 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 23 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 25 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.27% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.27% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

