# JVM hotspots (JFR ExecutionSample)

Samples: 9778

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 7.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 4 | 3.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 5 | 3.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.23% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15364` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 10 | 1.83% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.52% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.43% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 14 | 1.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5896` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6183` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3154` |
| 18 | 0.96% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6512` |
| 21 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14688` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.82% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 25 | 0.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11806` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6119` |
| 31 | 0.61% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3150` |
| 32 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 33 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 35 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10147` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5900` |
| 37 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 38 | 0.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1031` |
| 39 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3149` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12934` |
| 43 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6521` |
| 45 | 0.37% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 46 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.35% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2590` |
| 49 | 0.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 51 | 0.28% | `dev.planeta9091.centities.CeHooks.standCheckActive(ArmorStand) line: 5806` |
| 52 | 0.28% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6192` |
| 54 | 0.26% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1159` |
| 55 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2624` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9174` |
| 57 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9175` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 60 | 0.21% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15623` |
| 3 | 7.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5897` |
| 5 | 3.95% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 6 | 3.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10138` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6073` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13119` |
| 11 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14485` |
| 13 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13052` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6247` |
| 16 | 1.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 17 | 1.21% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6160` |
| 20 | 1.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15349` |
| 21 | 1.08% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6131` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15661` |
| 25 | 0.83% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14494` |
| 26 | 0.80% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11813` |
| 27 | 0.80% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5898` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15255` |
| 29 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10146` |

GC pauses: 0, total 0 ms

