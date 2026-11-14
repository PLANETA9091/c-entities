# JVM hotspots (JFR ExecutionSample)

Samples: 2014

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12511` |
| 2 | 3.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 1.89% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12530` |
| 14 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.89% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 16 | 0.79% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 17 | 0.79% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 18 | 0.74% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 19 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 20 | 0.60% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 21 | 0.60% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 23 | 0.55% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 24 | 0.55% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.50% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 26 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.40% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9604` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8034` |
| 33 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.35% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 35 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 37 | 0.35% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 38 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 39 | 0.35% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 40 | 0.35% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 41 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.30% | `jdk.internal.misc.Unsafe.allocateInstance(Class)` |
| 43 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 45 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 47 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.25% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 49 | 0.25% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 50 | 0.25% | `java.util.Collection.stream() line: 748` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 52 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 54 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.25% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 56 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 57 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.20% | `net.minecraft.nbt.CompoundTag.<init>() line: 195` |
| 60 | 0.20% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 2 | 3.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 3.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10917` |
| 5 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10850` |
| 7 | 1.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.24% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10870` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5569` |
| 17 | 0.89% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 18 | 0.79% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 19 | 0.74% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 20 | 0.74% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.70% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6656` |
| 23 | 0.60% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.50% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10878` |
| 29 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.40% | `java.util.zip.ZipFile.getEntry(String) line: 294` |

GC pauses: 0, total 0 ms

