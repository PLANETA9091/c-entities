# JVM hotspots (JFR ExecutionSample)

Samples: 15569

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 2 | 10.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 3 | 5.48% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 5 | 3.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5147` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 9 | 2.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 10 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.22% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2571` |
| 17 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2568` |
| 19 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7478` |
| 21 | 0.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12795` |
| 23 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12841` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12782` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9888` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 31 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2570` |
| 35 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 37 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12799` |
| 38 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.30% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 40 | 0.28% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 41 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1923` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11016` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7434` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1903` |
| 47 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12823` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1930` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2573` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7296` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2481` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5284` |
| 55 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8291` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2057` |
| 58 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8317` |
| 60 | 0.13% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 4 | 5.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5836` |
| 5 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 8 | 2.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 9 | 2.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 2.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5258` |
| 11 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11201` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11134` |
| 14 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7420` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8322` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13011` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13019` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9895` |
| 25 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.49% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5142` |
| 28 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 29 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2457` |

GC pauses: 0, total 0 ms

