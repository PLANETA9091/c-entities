# JVM hotspots (JFR ExecutionSample)

Samples: 8726

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 2 | 4.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9635` |
| 3 | 3.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 5 | 2.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 6 | 2.13% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 8 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6814` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9286` |
| 12 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13967` |
| 14 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.83% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6073` |
| 19 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16642` |
| 21 | 0.64% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 22 | 0.61% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 23 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3284` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16457` |
| 28 | 0.44% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.41% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 32 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.38% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 14012` |
| 34 | 0.37% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6077` |
| 35 | 0.36% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1071` |
| 36 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.34% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6823` |
| 39 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2714` |
| 40 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10801` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13955` |
| 42 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 930` |
| 43 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10899` |
| 45 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9292` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseDelta(ItemEntity, int) line: 1074` |
| 47 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 50 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 52 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1160` |
| 53 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6813` |
| 55 | 0.22% | `java.util.Collection.stream() line: 748` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1056` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10882` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6362` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3280` |
| 60 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 3.12% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6074` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 5 | 2.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16627` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8553` |
| 8 | 2.11% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6549` |
| 9 | 1.38% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15351` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9296` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6275` |
| 12 | 1.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10886` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9292` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1056` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10929` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15360` |
| 20 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.68% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6317` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13878` |
| 26 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.52% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 28 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16635` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |

GC pauses: 0, total 0 ms

