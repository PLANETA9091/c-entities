# JVM hotspots (JFR ExecutionSample)

Samples: 6627

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 6.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9073` |
| 4 | 4.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 5 | 2.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8784` |
| 7 | 2.08% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6541` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.43% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.31% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5904` |
| 15 | 1.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 16 | 1.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 1.21% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 1.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15459` |
| 19 | 0.92% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10242` |
| 21 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 24 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 25 | 0.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3162` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14783` |
| 30 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6128` |
| 32 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 33 | 0.51% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11901` |
| 35 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8758` |
| 36 | 0.47% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5908` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 39 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10205` |
| 41 | 0.35% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 42 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 44 | 0.33% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3157` |
| 46 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6550` |
| 47 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8663` |
| 48 | 0.32% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8640` |
| 51 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2595` |
| 53 | 0.29% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3167` |
| 54 | 0.27% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1168` |
| 55 | 0.27% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6212` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9204` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5997` |
| 60 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 13029` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.78% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15718` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 5 | 2.90% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5905` |
| 6 | 2.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14580` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6106` |
| 9 | 1.90% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6276` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6160` |
| 11 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14589` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13214` |
| 14 | 1.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13147` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 19 | 0.83% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5906` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8762` |
| 22 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6128` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10236` |
| 25 | 0.65% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6851` |
| 27 | 0.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15444` |
| 29 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

