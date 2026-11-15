# JVM hotspots (JFR ExecutionSample)

Samples: 12013

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 2 | 9.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 3 | 7.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 4 | 5.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 3.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.70% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 2.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.77% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.52% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.40% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12958` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.71% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12819` |
| 24 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9895` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13004` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 31 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12962` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7436` |
| 34 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5196` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11023` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 45 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1924` |
| 48 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 50 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 52 | 0.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 53 | 0.14% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 54 | 0.14% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 55 | 0.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8329` |
| 56 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8324` |
| 58 | 0.13% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 59 | 0.13% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.12% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 7 | 3.78% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 8 | 2.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 2.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13209` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11208` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11141` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 18 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8329` |
| 20 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.70% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13196` |
| 24 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8325` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9902` |
| 29 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6858` |

GC pauses: 0, total 0 ms

