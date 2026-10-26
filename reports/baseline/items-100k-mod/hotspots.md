# JVM hotspots (JFR ExecutionSample)

Samples: 3322

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 4.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 4 | 3.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 5 | 3.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 7 | 2.95% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.99% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4032` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 15 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2104` |
| 17 | 1.08% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2213` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 25 | 0.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 27 | 0.42% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 28 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 31 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 33 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 36 | 0.24% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2061` |
| 38 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 41 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 42 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 43 | 0.18% | `ca.spottedleaf.concurrentutil.executor.thread.BalancedPrioritisedThreadPool$OrderedStreamGroup$Queue.createTask(Runnable, Priority, long, long) line: 464` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2115` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2013` |
| 46 | 0.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 47 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 49 | 0.15% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.15% | `net.minecraft.world.level.levelgen.synth.SmearedPerlinNoise.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 216` |
| 51 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 52 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |
| 54 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 55 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3533` |
| 57 | 0.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 58 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 59 | 0.15% | `net.minecraft.world.entity.Entity.getBukkitEntity() line: 209` |
| 60 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.49% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 9.54% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 3 | 8.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 4 | 4.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 4.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 7 | 3.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2018` |
| 8 | 2.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 2.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 10 | 1.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.96% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6421` |
| 12 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 13 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 15 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 17 | 1.05% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2136` |
| 20 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2098` |
| 23 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.36% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 27 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 29 | 0.33% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 30 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

