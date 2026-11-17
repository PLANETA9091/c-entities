# JVM hotspots (JFR ExecutionSample)

Samples: 10194

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 2 | 9.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 4.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 4 | 4.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8530` |
| 6 | 3.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7567` |
| 10 | 1.62% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14392` |
| 13 | 1.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 14 | 1.23% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8316` |
| 16 | 1.10% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5860` |
| 19 | 0.95% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 20 | 0.95% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5864` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14374` |
| 23 | 0.84% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5818` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2936` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 28 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10983` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9383` |
| 33 | 0.49% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 34 | 0.41% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 35 | 0.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7573` |
| 36 | 0.37% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7558` |
| 37 | 0.36% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7571` |
| 38 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5754` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12111` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2376` |
| 42 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8312` |
| 44 | 0.32% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 45 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2935` |
| 46 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.30% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2940` |
| 49 | 0.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14846` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7585` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2537` |
| 53 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 56 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14385` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2410` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9398` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 10.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 4 | 6.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 3.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7663` |
| 6 | 3.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9383` |
| 7 | 2.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12296` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12229` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8294` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13662` |
| 14 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9403` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5602` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5766` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14377` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9399` |
| 26 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13671` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10990` |
| 30 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

