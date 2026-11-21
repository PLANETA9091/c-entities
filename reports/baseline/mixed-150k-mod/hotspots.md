# JVM hotspots (JFR ExecutionSample)

Samples: 9377

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 7.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 3 | 6.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 7 | 2.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 10 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15556` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 18 | 0.97% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 22 | 0.76% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11984` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 29 | 0.50% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5928` |
| 31 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 33 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 34 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 37 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 39 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2599` |
| 41 | 0.35% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 42 | 0.32% | `java.util.ArrayList$Itr.next() line: 1058` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6271` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2627` |
| 46 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 48 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 51 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 52 | 0.19% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6200` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.standCheckActive(ArmorStand) line: 5834` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8567` |
| 56 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 57 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntityStatus(Entity) line: 401` |
| 58 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.17% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 60 | 0.17% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 3 | 7.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.97% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5925` |
| 5 | 3.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.93% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 7 | 2.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6126` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13297` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13230` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14663` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10324` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14672` |
| 16 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.11% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15853` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 23 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15447` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11991` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15541` |
| 30 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |

GC pauses: 0, total 0 ms

