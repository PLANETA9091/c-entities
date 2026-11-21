# JVM hotspots (JFR ExecutionSample)

Samples: 3366

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 2 | 4.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 4 | 2.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 6 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10286` |
| 7 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.92% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 0.83% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.77% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 17 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.53% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 19 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 20 | 0.53% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 21 | 0.51% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 22 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 23 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.45% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 27 | 0.45% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 28 | 0.42% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 30 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2627` |
| 32 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 35 | 0.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 36 | 0.36% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 37 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 41 | 0.33% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 42 | 0.30% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 43 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 44 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 45 | 0.27% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 46 | 0.27% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 47 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.24% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 50 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 51 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 52 | 0.21% | `com.viaversion.viaversion.api.data.FullMappingsBase.toInverseMap(List) line: 65` |
| 53 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.21% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 12049` |
| 56 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 57 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 58 | 0.18% | `java.util.Objects.hashCode(Object) line: 97` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 60 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 2 | 4.07% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13314` |
| 3 | 4.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13381` |
| 4 | 3.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 9 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14750` |
| 13 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8168` |
| 14 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10353` |
| 15 | 0.80% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 0.74% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.74% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 22 | 0.53% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15628` |
| 24 | 0.51% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 25 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.45% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13334` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15940` |
| 30 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |

GC pauses: 0, total 0 ms

