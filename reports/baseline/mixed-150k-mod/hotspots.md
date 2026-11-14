# JVM hotspots (JFR ExecutionSample)

Samples: 10187

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 2 | 7.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 3 | 6.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 4 | 6.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 8 | 3.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4981` |
| 11 | 1.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.42% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 18 | 0.93% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 19 | 0.88% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.67% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12819` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12680` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7303` |
| 31 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5155` |
| 32 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5422` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12865` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9756` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 36 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5156` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12823` |
| 39 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2043` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 43 | 0.27% | `java.util.ArrayList$Itr.next() line: 1058` |
| 44 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 46 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10884` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1915` |
| 51 | 0.24% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1885` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7142` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 341` |
| 55 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.22% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 57 | 0.20% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 59 | 0.20% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB) line: 4745` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 10.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 8.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5167` |
| 6 | 3.91% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.21% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5676` |
| 8 | 2.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5094` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13070` |
| 11 | 1.56% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5118` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11002` |
| 16 | 1.22% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 1.17% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11069` |
| 18 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8190` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7289` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7311` |
| 24 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.67% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13057` |
| 28 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9763` |
| 30 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

