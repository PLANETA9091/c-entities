# JVM hotspots (JFR ExecutionSample)

Samples: 3629

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 2 | 7.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 3 | 4.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 5 | 3.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 7 | 2.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6321` |
| 11 | 2.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5548` |
| 12 | 1.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 13 | 1.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9615` |
| 14 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 16 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5652` |
| 17 | 1.18% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 18 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5619` |
| 22 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9568` |
| 23 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9571` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5604` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 27 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5602` |
| 30 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 31 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.30% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 33 | 0.30% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2730` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5490` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9791` |
| 38 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3943` |
| 40 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4025` |
| 44 | 0.22% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 45 | 0.22% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 46 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 50 | 0.19% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 52 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9576` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4027` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4026` |
| 56 | 0.17% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 58 | 0.17% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 59 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2277` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 3 | 6.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9745` |
| 4 | 4.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 5 | 4.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 6 | 4.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6321` |
| 7 | 4.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 8 | 2.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 13 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5604` |
| 15 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3951` |
| 17 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 23 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 24 | 0.28% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5623` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.28% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 30 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

