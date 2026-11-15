# JVM hotspots (JFR ExecutionSample)

Samples: 6622

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 5.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 3 | 5.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 5 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 6 | 3.85% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 7 | 2.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 11 | 1.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 18 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.77% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 20 | 0.68% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.66% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 23 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 24 | 0.63% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 25 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 30 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.44% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 32 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 33 | 0.42% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 35 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 37 | 0.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9881` |
| 39 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 40 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 41 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5133` |
| 44 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 46 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.21% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |
| 49 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 50 | 0.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 51 | 0.21% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 53 | 0.20% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 54 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 56 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 58 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 60 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 8.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 6 | 3.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.80% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8315` |
| 10 | 1.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 11 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.03% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.79% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 19 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.66% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 22 | 0.63% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5004` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11147` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |

GC pauses: 0, total 0 ms

