# JVM hotspots (JFR ExecutionSample)

Samples: 4428

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 3 | 4.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 4 | 3.93% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 5 | 3.36% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 7 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 9 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2978` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.88% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 17 | 0.86% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 20 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 21 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1187` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 25 | 0.43% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 27 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 30 | 0.32% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 31 | 0.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 33 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3015` |
| 36 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 38 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2984` |
| 40 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3516` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2025` |
| 44 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2054` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2093` |
| 47 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 49 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3672` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3479` |
| 53 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 57 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 58 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 60 | 0.16% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 10.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 3 | 5.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3672` |
| 4 | 3.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 3.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 6 | 3.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 8 | 2.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 2.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3015` |
| 11 | 1.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 12 | 0.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 0.95% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 14 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 15 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4470` |
| 19 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 21 | 0.54% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 24 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 26 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.32% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.29% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 30 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

