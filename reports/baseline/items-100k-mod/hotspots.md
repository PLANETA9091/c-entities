# JVM hotspots (JFR ExecutionSample)

Samples: 3707

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 11.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 3 | 5.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 5 | 2.94% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.72% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3493` |
| 17 | 0.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 19 | 0.59% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2120` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2218` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2109` |
| 25 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4037` |
| 29 | 0.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 31 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 32 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3530` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3499` |
| 35 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 36 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 39 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 40 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.22% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 42 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 45 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 476` |
| 47 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 623` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2025` |
| 50 | 0.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2066` |
| 52 | 0.13% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1873` |
| 53 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1616` |
| 55 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.13% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6423` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3538` |
| 58 | 0.13% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 59 | 0.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 60 | 0.11% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.71% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 12.19% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 3 | 6.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 5 | 4.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 8 | 2.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3530` |
| 9 | 2.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 2.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6608` |
| 14 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 18 | 0.73% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 19 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.59% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2141` |
| 22 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 24 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2103` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4037` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

