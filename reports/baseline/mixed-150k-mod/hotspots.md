# JVM hotspots (JFR ExecutionSample)

Samples: 11551

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 7.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7913` |
| 3 | 6.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 5 | 5.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 6 | 3.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5388` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7717` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.06% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13681` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12991` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13663` |
| 19 | 0.86% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.83% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 25 | 0.60% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 26 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5342` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8008` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 32 | 0.41% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 33 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5390` |
| 35 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13918` |
| 36 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13496` |
| 37 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13635` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 40 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 41 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2109` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 44 | 0.25% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10320` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5260` |
| 46 | 0.24% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13639` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7511` |
| 49 | 0.23% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11448` |
| 51 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 53 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |
| 55 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntityStatus(Entity) line: 401` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13979` |
| 57 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7721` |
| 59 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13674` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 11.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 4 | 6.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 6 | 3.13% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5933` |
| 7 | 2.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 9 | 1.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13008` |
| 11 | 1.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.20% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 13 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.03% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11633` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12999` |
| 17 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8754` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11566` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8750` |
| 21 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 22 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13909` |
| 26 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5108` |
| 28 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7699` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |

GC pauses: 0, total 0 ms

