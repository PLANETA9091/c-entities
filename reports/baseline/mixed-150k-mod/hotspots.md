# JVM hotspots (JFR ExecutionSample)

Samples: 12637

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 3 | 7.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 4 | 5.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4904` |
| 9 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 11 | 1.62% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.42% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5070` |
| 14 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.01% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12490` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12502` |
| 21 | 0.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12548` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 28 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9604` |
| 31 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7195` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7151` |
| 34 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 36 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1299` |
| 37 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12506` |
| 39 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10732` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7127` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 42 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5329` |
| 45 | 0.21% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 46 | 0.19% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 47 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 48 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2043` |
| 50 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7155` |
| 52 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 54 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 55 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.13% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 57 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.13% | `java.lang.ThreadLocal.get() line: 171` |
| 59 | 0.13% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 60 | 0.13% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.43% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5569` |
| 7 | 2.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5015` |
| 8 | 2.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5078` |
| 9 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 11 | 1.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10917` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10850` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7137` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8038` |
| 20 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12722` |
| 22 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9611` |
| 25 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5039` |
| 27 | 0.51% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8034` |
| 30 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

