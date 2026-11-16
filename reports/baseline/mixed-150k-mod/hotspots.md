# JVM hotspots (JFR ExecutionSample)

Samples: 11168

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 8.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 3 | 6.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 4 | 5.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.17% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 11 | 1.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 14 | 1.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.33% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 0.92% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 18 | 0.86% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2631` |
| 20 | 0.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13809` |
| 26 | 0.73% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10494` |
| 31 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 32 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2635` |
| 34 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 35 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13813` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2109` |
| 37 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 38 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5477` |
| 40 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5395` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 43 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7718` |
| 44 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.30% | `java.util.ArrayList$Itr.next() line: 1058` |
| 47 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11622` |
| 49 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 50 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7856` |
| 51 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 54 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5486` |
| 56 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 57 | 0.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7868` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14153` |
| 60 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 10.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 9.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 9.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 5 | 5.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 7 | 2.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 10 | 1.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13173` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5407` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8928` |
| 18 | 0.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5243` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14083` |
| 22 | 0.81% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13182` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10501` |
| 24 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8924` |
| 29 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms

