# JVM hotspots (JFR ExecutionSample)

Samples: 12901

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 2 | 9.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 3 | 7.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 4 | 4.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.59% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 11 | 1.77% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |
| 16 | 1.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12883` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12744` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 24 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12929` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9820` |
| 29 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8223` |
| 32 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7367` |
| 34 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12887` |
| 35 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 36 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2IntOpenHashMap.containsKey(Object) line: 383` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8249` |
| 39 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 41 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7343` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5204` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 46 | 0.19% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 47 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10948` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7371` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 54 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 56 | 0.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 58 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 59 | 0.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2043` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 6.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.29% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 7 | 4.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 9 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13134` |
| 11 | 1.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7353` |
| 13 | 1.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11066` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11133` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8254` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13121` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.59% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 24 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 26 | 0.46% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9827` |
| 27 | 0.39% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12917` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7349` |

GC pauses: 0, total 0 ms

