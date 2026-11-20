# JVM hotspots (JFR ExecutionSample)

Samples: 6851

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 3 | 4.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8849` |
| 5 | 3.12% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.97% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 7 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15145` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14282` |
| 9 | 1.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6116` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1064` |
| 12 | 1.01% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.99% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.89% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14481` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 19 | 0.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5787` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3041` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8585` |
| 23 | 0.69% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 26 | 0.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5783` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 29 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 30 | 0.54% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 31 | 0.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 32 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6112` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 34 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8581` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6006` |
| 38 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 39 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 41 | 0.38% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3037` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.35% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 45 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9932` |
| 46 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8555` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3036` |
| 50 | 0.25% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1021` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6079` |
| 53 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 56 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6118` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 3 | 5.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.03% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 5 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 6 | 3.04% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5784` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6018` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1062` |
| 9 | 1.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5960` |
| 11 | 1.27% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6047` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9951` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14299` |
| 16 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12924` |
| 19 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 0.89% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15442` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5785` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.mergeMaxStack(Object) line: 913` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5786` |
| 27 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.66% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 29 | 0.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12877` |

GC pauses: 0, total 0 ms

