# JVM hotspots (JFR ExecutionSample)

Samples: 10216

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 6.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 3 | 5.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 7 | 1.51% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.47% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.37% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 15 | 0.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 18 | 0.89% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.75% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 23 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5928` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 28 | 0.49% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 30 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 31 | 0.47% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 32 | 0.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10344` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 35 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1034` |
| 36 | 0.36% | `java.util.ArrayList$Itr.next() line: 1058` |
| 37 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10274` |
| 38 | 0.32% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 39 | 0.32% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 41 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 42 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 43 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 45 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 46 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2760` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 49 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6271` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8567` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15886` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2599` |
| 54 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 55 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6180` |
| 57 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.19% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 59 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.18% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 3 | 6.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5925` |
| 5 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6126` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13314` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14750` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13381` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15940` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10374` |
| 18 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14759` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15534` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15628` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.61% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5926` |
| 27 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 28 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

