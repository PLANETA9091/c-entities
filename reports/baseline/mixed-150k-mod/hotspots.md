# JVM hotspots (JFR ExecutionSample)

Samples: 12233

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 2 | 10.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 3 | 7.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 4 | 5.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.92% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 11 | 1.57% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.37% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.96% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 17 | 0.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 21 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12877` |
| 23 | 0.56% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12738` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12923` |
| 26 | 0.54% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7361` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12881` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 33 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 34 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9814` |
| 36 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8228` |
| 38 | 0.25% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 39 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7337` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 42 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.20% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 44 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 45 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 46 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2043` |
| 48 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 49 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.15% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10942` |
| 51 | 0.14% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 52 | 0.14% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 53 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.13% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB) line: 4745` |
| 55 | 0.13% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 56 | 0.12% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 57 | 0.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 58 | 0.12% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 59 | 0.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5165` |
| 60 | 0.12% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8228` |
| 7 | 4.02% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 8 | 2.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 9 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 11 | 1.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11060` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8248` |
| 18 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 22 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13115` |
| 25 | 0.54% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9821` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8244` |

GC pauses: 0, total 0 ms

