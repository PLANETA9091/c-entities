# JVM hotspots (JFR ExecutionSample)

Samples: 13252

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 2 | 8.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 3 | 5.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 4 | 4.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 1.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13026` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7440` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.98% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 18 | 0.96% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.86% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 21 | 0.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.62% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9917` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5196` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12841` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12980` |
| 32 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8331` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.38% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 36 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 39 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12984` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11045` |
| 43 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.27% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 46 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.26% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 48 | 0.26% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 51 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2454` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8330` |
| 55 | 0.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 57 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 60 | 0.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.29% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 6 | 4.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8331` |
| 7 | 3.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13231` |
| 9 | 2.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 11 | 1.71% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 12 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11163` |
| 15 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7400` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11230` |
| 18 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8351` |
| 20 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9924` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13218` |
| 25 | 0.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8347` |
| 27 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7374` |
| 30 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |

GC pauses: 0, total 0 ms

