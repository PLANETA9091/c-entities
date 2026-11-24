# JVM hotspots (JFR ExecutionSample)

Samples: 8800

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 2 | 7.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9497` |
| 5 | 3.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16262` |
| 8 | 1.52% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.48% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6025` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9243` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.01% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10842` |
| 18 | 0.97% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9215` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15432` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.82% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 25 | 0.81% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 29 | 0.73% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3232` |
| 31 | 0.67% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8932` |
| 34 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 35 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 36 | 0.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 37 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.39% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10811` |
| 40 | 0.38% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.36% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 42 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 43 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2672` |
| 44 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6314` |
| 46 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3231` |
| 49 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 50 | 0.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 52 | 0.28% | `java.util.ArrayList$Itr.next() line: 1058` |
| 53 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2700` |
| 55 | 0.26% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2833` |
| 56 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2706` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6294` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9678` |
| 60 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 3 | 7.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.82% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6026` |
| 5 | 3.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 6 | 3.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6227` |
| 9 | 1.69% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13781` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15217` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13848` |
| 14 | 1.38% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9215` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6501` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16247` |
| 18 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6269` |
| 19 | 0.97% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16581` |
| 22 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15226` |
| 25 | 0.81% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6027` |
| 28 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16250` |
| 30 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms

