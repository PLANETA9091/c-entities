# JVM hotspots (JFR ExecutionSample)

Samples: 3586

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 2 | 10.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 5 | 4.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 6 | 2.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.87% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4042` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3489` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2105` |
| 15 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4033` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2214` |
| 18 | 0.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.75% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.50% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 27 | 0.42% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3495` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3526` |
| 32 | 0.33% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 33 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2116` |
| 35 | 0.31% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 37 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 38 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 42 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4201` |
| 44 | 0.17% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 45 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2062` |
| 46 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.14% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.14% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6270` |
| 50 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.14% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2040` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2167` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2015` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2033` |
| 57 | 0.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 58 | 0.11% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.11% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.20% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 9.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 3 | 8.17% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 4 | 5.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 5.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3526` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2019` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6455` |
| 10 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 13 | 1.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 14 | 1.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2137` |
| 16 | 0.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4003` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2099` |
| 19 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 22 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 24 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.42% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 30 | 0.31% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |

GC pauses: 0, total 0 ms

