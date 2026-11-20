# JVM hotspots (JFR ExecutionSample)

Samples: 9627

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 2 | 4.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8984` |
| 3 | 4.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 5 | 2.45% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.87% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 8 | 1.55% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 12 | 1.32% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8673` |
| 14 | 1.09% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5867` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14628` |
| 19 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 20 | 0.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 23 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3125` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 26 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5871` |
| 31 | 0.46% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11746` |
| 33 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15304` |
| 35 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6090` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6209` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8679` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6494` |
| 40 | 0.32% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 41 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 43 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10078` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6154` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 47 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 48 | 0.26% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1026` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1031` |
| 52 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3121` |
| 54 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2722` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12874` |
| 57 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10081` |
| 60 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15563` |
| 4 | 3.51% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5868` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 6 | 2.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6220` |
| 8 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10078` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13059` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8683` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12992` |
| 13 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14425` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14434` |
| 16 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10086` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15601` |
| 19 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8679` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6102` |
| 22 | 0.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.48% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 29 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11753` |

GC pauses: 0, total 0 ms

