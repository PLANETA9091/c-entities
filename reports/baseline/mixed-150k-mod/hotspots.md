# JVM hotspots (JFR ExecutionSample)

Samples: 9824

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 2 | 8.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 4 | 3.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8660` |
| 5 | 3.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.85% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 8 | 1.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.51% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14546` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8446` |
| 13 | 1.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.28% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5990` |
| 16 | 0.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5994` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5661` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11113` |
| 21 | 0.80% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5948` |
| 24 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2955` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5665` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7703` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14528` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 34 | 0.45% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7688` |
| 35 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 36 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8442` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5996` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7701` |
| 39 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5884` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15000` |
| 42 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 44 | 0.32% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7715` |
| 46 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5754` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 50 | 0.27% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2959` |
| 52 | 0.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 53 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12241` |
| 55 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5957` |
| 57 | 0.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8424` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2395` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14539` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 3 | 6.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7793` |
| 5 | 4.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 6 | 3.71% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5662` |
| 7 | 3.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5838` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12359` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13792` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12426` |
| 13 | 1.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8424` |
| 16 | 1.14% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9533` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14531` |
| 20 | 0.96% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11120` |
| 23 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13801` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5663` |
| 28 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5896` |
| 30 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

