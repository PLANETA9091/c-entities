# JVM hotspots (JFR ExecutionSample)

Samples: 12743

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 9.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 5.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 10 | 1.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.25% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13856` |
| 18 | 0.81% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2635` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 30 | 0.52% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 31 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 32 | 0.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7765` |
| 34 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5533` |
| 35 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 36 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 38 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2083` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10541` |
| 40 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13860` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2639` |
| 43 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14200` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 47 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 48 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5844` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13853` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8975` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8944` |
| 54 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 55 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2109` |
| 57 | 0.16% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5484` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntityStatus(Entity) line: 401` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.62% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 9.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 8.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 5 | 4.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 6 | 3.85% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 7 | 2.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 14 | 1.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8975` |
| 16 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14130` |
| 18 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13229` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5290` |
| 24 | 0.60% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.57% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.39% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10548` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7299` |

GC pauses: 0, total 0 ms

