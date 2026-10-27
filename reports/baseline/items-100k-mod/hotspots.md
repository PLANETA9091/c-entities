# JVM hotspots (JFR ExecutionSample)

Samples: 2767

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 2 | 6.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 3 | 6.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 3.51% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2087` |
| 7 | 3.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2133` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 11 | 1.63% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2162` |
| 13 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |
| 16 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3546` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7146` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2173` |
| 20 | 0.72% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2271` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1463` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 25 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2119` |
| 27 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3583` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7153` |
| 30 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4257` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3591` |
| 34 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.29% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 37 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3552` |
| 38 | 0.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 39 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 40 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 41 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 42 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 43 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 46 | 0.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 47 | 0.22% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.set(VarHandle, Object, int) line: 146` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6838` |
| 49 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7125` |
| 51 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 53 | 0.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 54 | 0.18% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2071` |
| 56 | 0.18% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 57 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2224` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7156` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2091` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 10.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4257` |
| 3 | 8.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7191` |
| 4 | 3.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3583` |
| 5 | 3.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 6 | 2.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2087` |
| 8 | 2.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7149` |
| 9 | 2.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1079` |
| 10 | 2.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7183` |
| 11 | 2.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2088` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7141` |
| 17 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.72% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2194` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 21 | 0.58% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2156` |
| 23 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 26 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2173` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 30 | 0.33% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

