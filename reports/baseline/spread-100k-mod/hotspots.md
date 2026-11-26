# JVM hotspots (JFR ExecutionSample)

Samples: 6899

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 2 | 5.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9580` |
| 4 | 3.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 2.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16577` |
| 9 | 1.87% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.55% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.35% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.32% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.discard(EntityRemoveEvent$Cause) line: 685` |
| 16 | 1.09% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 17 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6031` |
| 19 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 20 | 0.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 0.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6320` |
| 23 | 0.72% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3242` |
| 25 | 0.67% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13900` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 27 | 0.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 28 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 32 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6781` |
| 34 | 0.45% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6035` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6275` |
| 37 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.36% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 41 | 0.36% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 42 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 43 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9711` |
| 46 | 0.29% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1224` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9129` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16526` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3247` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2675` |
| 54 | 0.23% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.offer(Object) line: 187` |
| 55 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 57 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10844` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8965` |
| 60 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8988` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.28% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 4.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16836` |
| 4 | 3.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 5 | 3.29% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6032` |
| 6 | 3.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16562` |
| 7 | 2.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6507` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15296` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6233` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15305` |
| 12 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6275` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8510` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13837` |
| 16 | 1.09% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 17 | 1.03% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13823` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10874` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 21 | 0.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6034` |
| 23 | 0.75% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9252` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6033` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

