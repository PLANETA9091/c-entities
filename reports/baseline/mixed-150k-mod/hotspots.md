# JVM hotspots (JFR ExecutionSample)

Samples: 15216

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 3 | 5.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 4.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 8 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 10 | 1.41% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 13 | 1.29% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 0.85% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14107` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10746` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.55% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 29 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.49% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.48% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 32 | 0.48% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2844` |
| 33 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5738` |
| 34 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 35 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 39 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |
| 41 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6049` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14058` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14065` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2308` |
| 48 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5647` |
| 49 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5729` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7898` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8080` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2314` |
| 54 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14405` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9175` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5779` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2288` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 12.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 5 | 5.63% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 6 | 3.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 7 | 2.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 11 | 1.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 12 | 0.95% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9180` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14335` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13434` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10753` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5495` |
| 27 | 0.51% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14095` |

GC pauses: 0, total 0 ms

