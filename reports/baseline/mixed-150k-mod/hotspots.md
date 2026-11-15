# JVM hotspots (JFR ExecutionSample)

Samples: 14049

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 7.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 6.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 5 | 5.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.99% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.63% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 17 | 1.07% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.93% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 22 | 0.75% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 23 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12944` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 30 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 31 | 0.48% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 32 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 33 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 35 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 36 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 37 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9881` |
| 39 | 0.30% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.27% | `java.util.ArrayList.<init>(Collection) line: 184` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11009` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12948` |
| 43 | 0.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 44 | 0.23% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5182` |
| 47 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 49 | 0.19% | `java.util.ArrayList$Itr.next() line: 1058` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 51 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8310` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 55 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 56 | 0.16% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7099` |
| 58 | 0.15% | `net.minecraft.world.entity.Entity.makeBoundingBox(Vec3) line: 841` |
| 59 | 0.15% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 60 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 8.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 7.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.44% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 6 | 3.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 7 | 3.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 10 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 11 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.65% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.07% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 18 | 0.93% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8315` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8311` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7386` |
| 23 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13182` |
| 26 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |

GC pauses: 0, total 0 ms

