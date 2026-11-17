# JVM hotspots (JFR ExecutionSample)

Samples: 8812

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8580` |
| 6 | 3.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 7 | 3.21% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5864` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14392` |
| 13 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5860` |
| 14 | 0.93% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.85% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8312` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14374` |
| 18 | 0.76% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8368` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7567` |
| 21 | 0.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 23 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5754` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7571` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 984` |
| 27 | 0.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 28 | 0.59% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 29 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 31 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5866` |
| 32 | 0.49% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 33 | 0.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.47% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 35 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9383` |
| 37 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.39% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 40 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.31% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 42 | 0.31% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7573` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5818` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 46 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 47 | 0.26% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 48 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7558` |
| 52 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10983` |
| 54 | 0.22% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14346` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14385` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2935` |
| 58 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5670` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 6.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 5 | 4.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9383` |
| 6 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5766` |
| 7 | 2.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7663` |
| 8 | 1.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 10 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13671` |
| 12 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 13 | 0.81% | `dev.planeta9091.centities.CeHooks.mergeMaxStack(Object) line: 879` |
| 14 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9403` |
| 16 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.70% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12229` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12296` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 21 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13662` |
| 23 | 0.57% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8298` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5602` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12249` |
| 30 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

