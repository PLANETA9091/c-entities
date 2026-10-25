# JVM hotspots (JFR ExecutionSample)

Samples: 4061

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3467` |
| 3 | 8.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 4 | 6.82% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4363` |
| 5 | 3.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2059` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 7 | 2.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.51% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 9 | 2.04% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2147` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 13 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2085` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3494` |
| 17 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2962` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2194` |
| 20 | 0.69% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.57% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 25 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 27 | 0.42% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3615` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1424` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1405` |
| 32 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.34% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 34 | 0.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3478` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 37 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1408` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2096` |
| 40 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2013` |
| 41 | 0.30% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 42 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3500` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2968` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2143` |
| 46 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.17% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 50 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 52 | 0.15% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 53 | 0.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 54 | 0.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 55 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 56 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 57 | 0.12% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 58 | 0.12% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 140` |
| 59 | 0.12% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 60 | 0.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.52% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 10.22% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2647` |
| 3 | 9.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 7.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3615` |
| 5 | 6.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2999` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2010` |
| 9 | 2.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2046` |
| 10 | 1.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4364` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1058` |
| 15 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2117` |
| 18 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 20 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3477` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 698` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks$RefField.getInt(Object) line: 2623` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.34% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 29 | 0.34% | `net.minecraft.util.Mth.packDegrees(float) line: 183` |
| 30 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

GC pauses: 0, total 0 ms

