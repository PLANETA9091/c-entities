# JVM hotspots (JFR ExecutionSample)

Samples: 12514

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 2 | 10.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 3 | 7.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 4 | 5.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 5.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.71% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.40% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 17 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 23 | 0.68% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9931` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12994` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 29 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13040` |
| 32 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 33 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 34 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 35 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.33% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 39 | 0.30% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 40 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12998` |
| 41 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 43 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 44 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.18% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 46 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 49 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8360` |
| 50 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 51 | 0.14% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5196` |
| 53 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.14% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 55 | 0.14% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 56 | 0.14% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB) line: 4745` |
| 57 | 0.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.13% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11059` |
| 60 | 0.13% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.52% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.40% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 7 | 3.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8365` |
| 18 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9938` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13232` |
| 23 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 28 | 0.46% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13028` |

GC pauses: 0, total 0 ms

