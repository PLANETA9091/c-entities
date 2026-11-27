# JVM hotspots (JFR ExecutionSample)

Samples: 8322

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 6.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 3 | 6.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 4 | 3.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.90% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16642` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9585` |
| 8 | 2.15% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6077` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3284` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9286` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3280` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6814` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 20 | 0.78% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6073` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1056` |
| 25 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 26 | 0.65% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 27 | 0.61% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 28 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6823` |
| 31 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.56% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1074` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13967` |
| 36 | 0.47% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16457` |
| 37 | 0.46% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6362` |
| 40 | 0.38% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.36% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9766` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10899` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13955` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9292` |
| 46 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2754` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3279` |
| 48 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.getSharedFlag$(Entity, int) line: 12451` |
| 51 | 0.26% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 53 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1071` |
| 54 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntityStatus(Entity) line: 401` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6166` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 2967` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 59 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6342` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 2 | 7.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 5 | 4.33% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6074` |
| 6 | 3.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6275` |
| 8 | 2.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6549` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9292` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15360` |
| 13 | 1.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10929` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1056` |
| 16 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8553` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6317` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1078` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.91% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1073` |
| 23 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13898` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15351` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13878` |
| 30 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

