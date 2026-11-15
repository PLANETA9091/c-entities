# JVM hotspots (JFR ExecutionSample)

Samples: 13441

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 2 | 9.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 3 | 6.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 4 | 4.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 8 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 11 | 1.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.26% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 15 | 1.08% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.73% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12944` |
| 21 | 0.62% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 26 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9881` |
| 30 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 31 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12948` |
| 33 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1299` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5494` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5133` |
| 39 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 41 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 43 | 0.22% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 45 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5221` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8284` |
| 49 | 0.19% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5063` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5217` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5182` |
| 54 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 55 | 0.16% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB) line: 4745` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11009` |
| 57 | 0.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.14% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.14% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 12.85% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 6.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.77% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 6 | 4.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 9 | 2.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 12 | 1.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8315` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.57% | `java.util.HashMap.containsKey(Object) line: 603` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13182` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 24 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9888` |
| 28 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 30 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

