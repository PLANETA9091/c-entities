# JVM hotspots (JFR ExecutionSample)

Samples: 11706

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 3 | 6.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 5 | 5.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 6 | 3.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5054` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2504` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7644` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13575` |
| 19 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 20 | 0.87% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 21 | 0.85% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10242` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13557` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13812` |
| 28 | 0.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2508` |
| 30 | 0.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 31 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13533` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13529` |
| 35 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7648` |
| 36 | 0.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 39 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13390` |
| 40 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7626` |
| 41 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5328` |
| 44 | 0.29% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7620` |
| 47 | 0.28% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 48 | 0.23% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 49 | 0.22% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5280` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |
| 52 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13873` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 55 | 0.20% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 56 | 0.18% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 57 | 0.18% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.17% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1956` |
| 60 | 0.17% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 10.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 4 | 6.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 6 | 3.03% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5871` |
| 7 | 2.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5210` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7626` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5152` |
| 11 | 1.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12930` |
| 13 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 1.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8676` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11555` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12921` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10249` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11488` |
| 23 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8672` |
| 28 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13803` |
| 30 | 0.38% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |

GC pauses: 0, total 0 ms

