# JVM hotspots (JFR ExecutionSample)

Samples: 11689

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 2 | 6.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8799` |
| 5 | 2.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14282` |
| 7 | 1.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 8 | 1.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 9 | 1.65% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.32% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5783` |
| 12 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8585` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14481` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15145` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6070` |
| 18 | 0.88% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3041` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6116` |
| 24 | 0.66% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 25 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6112` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3037` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11611` |
| 29 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.51% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5787` |
| 32 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.45% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 34 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 36 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 38 | 0.37% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12739` |
| 39 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3036` |
| 41 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9932` |
| 43 | 0.28% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2638` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 46 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 47 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 48 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 49 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 50 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2505` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 53 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6006` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9946` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 57 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6390` |
| 59 | 0.19% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8581` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 3 | 6.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5784` |
| 5 | 3.24% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 6 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 7 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5960` |
| 10 | 1.64% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12924` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14290` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12857` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9951` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15442` |
| 18 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6047` |
| 21 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15048` |
| 24 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14299` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6018` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1062` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9947` |
| 30 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

