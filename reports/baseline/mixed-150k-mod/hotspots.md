# JVM hotspots (JFR ExecutionSample)

Samples: 11814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 3 | 6.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 4 | 4.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8571` |
| 5 | 3.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7608` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 8 | 1.52% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.03% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.96% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5901` |
| 16 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14457` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5623` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14439` |
| 20 | 0.66% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 21 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5859` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2942` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5905` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8353` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7612` |
| 31 | 0.40% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 32 | 0.39% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7614` |
| 33 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 34 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 35 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 36 | 0.33% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7599` |
| 37 | 0.30% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11024` |
| 38 | 0.30% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9424` |
| 40 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2382` |
| 42 | 0.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 43 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8088` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5907` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12152` |
| 48 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 49 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8335` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2946` |
| 52 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14911` |
| 55 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9444` |
| 57 | 0.19% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 58 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 59 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14693` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14711` |
| 3 | 6.86% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.77% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5612` |
| 5 | 4.42% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5621` |
| 6 | 4.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7704` |
| 7 | 3.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9424` |
| 9 | 2.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12337` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12270` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13703` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9444` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8335` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5749` |
| 16 | 0.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13712` |
| 22 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5616` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5611` |
| 27 | 0.60% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5620` |
| 29 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8339` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14442` |

GC pauses: 0, total 0 ms

