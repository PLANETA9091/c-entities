# JVM hotspots (JFR ExecutionSample)

Samples: 10836

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 2 | 8.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 6.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 4 | 5.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7983` |
| 5 | 4.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7791` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13061` |
| 11 | 1.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.20% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5458` |
| 15 | 1.17% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 1.07% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.90% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13566` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2574` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13733` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13705` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10390` |
| 28 | 0.64% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13751` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7787` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8804` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5412` |
| 34 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.45% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2052` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13988` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5460` |
| 38 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13709` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14049` |
| 40 | 0.37% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2578` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7653` |
| 42 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.30% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 45 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.27% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11518` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5330` |
| 50 | 0.24% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 51 | 0.23% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5421` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8819` |
| 55 | 0.18% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.18% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7581` |
| 58 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2019` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8793` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 11.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 10.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 8.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 5 | 5.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8804` |
| 6 | 4.02% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6003` |
| 7 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11636` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13069` |
| 12 | 1.12% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 13 | 1.08% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11703` |
| 15 | 0.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.98% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8824` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13979` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5342` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5178` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13078` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10397` |
| 24 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8820` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13739` |

GC pauses: 0, total 0 ms

