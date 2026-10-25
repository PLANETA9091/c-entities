# JVM hotspots (JFR ExecutionSample)

Samples: 4096

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 3 | 6.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 4 | 5.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 5 | 4.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 6 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 11 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 13 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3114` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 16 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.88% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 22 | 0.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 24 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 27 | 0.37% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 28 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3151` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3120` |
| 36 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.24% | `java.util.Collection.stream() line: 748` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 39 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3808` |
| 41 | 0.22% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3645` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 44 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 45 | 0.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 47 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.17% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2155` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 5274` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3652` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2054` |
| 54 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 55 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3159` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2014` |
| 58 | 0.15% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 59 | 0.12% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.12% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.19% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 10.13% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 3 | 6.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 5.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3808` |
| 5 | 5.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 7 | 2.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3151` |
| 9 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 13 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 16 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5423` |
| 18 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 21 | 0.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 24 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 25 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.37% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2091` |
| 29 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

