# JVM hotspots (JFR ExecutionSample)

Samples: 11959

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 2 | 9.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7923` |
| 3 | 6.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 4 | 5.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5394` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 10 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.30% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13645` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2514` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.80% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13506` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5398` |
| 22 | 0.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.69% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13673` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2518` |
| 28 | 0.48% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10330` |
| 30 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.43% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11458` |
| 34 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7709` |
| 35 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7727` |
| 36 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8744` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13928` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13691` |
| 39 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7593` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13649` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5400` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1966` |
| 45 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8759` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13684` |
| 48 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 49 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7731` |
| 51 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.20% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13989` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5270` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5672` |
| 56 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7703` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1986` |
| 59 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 12.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 10.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 8.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 5 | 4.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8744` |
| 6 | 4.43% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5943` |
| 7 | 2.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7709` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11576` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11643` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13009` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8764` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13919` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.70% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5118` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13018` |
| 22 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 24 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8760` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10337` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5282` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7713` |
| 29 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.43% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

