# JVM hotspots (JFR ExecutionSample)

Samples: 2969

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12828` |
| 3 | 3.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 7 | 2.09% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.74% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12847` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 21 | 0.47% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 22 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.47% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 25 | 0.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 26 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.44% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 28 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11072` |
| 30 | 0.40% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 31 | 0.40% | `java.util.ArrayList$Itr.next() line: 1058` |
| 32 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12865` |
| 34 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11052` |
| 41 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 42 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 43 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.30% | `java.lang.Class.cast(Object) line: 3491` |
| 45 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 46 | 0.30% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 47 | 0.27% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 49 | 0.27% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 50 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 54 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9756` |
| 56 | 0.24% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 57 | 0.20% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 455` |
| 58 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 59 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 60 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13070` |
| 2 | 5.96% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11069` |
| 3 | 4.31% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11002` |
| 4 | 4.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.05% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6763` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8186` |
| 12 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5676` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.67% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.67% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12853` |
| 23 | 0.44% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 24 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 26 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 29 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.37% | `java.util.ArrayList.grow(int) line: 238` |

GC pauses: 0, total 0 ms

