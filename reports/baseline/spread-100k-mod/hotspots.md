# JVM hotspots (JFR ExecutionSample)

Samples: 9651

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9044` |
| 3 | 4.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 5 | 2.57% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 7 | 1.81% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 9 | 1.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 11 | 1.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.09% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6512` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14688` |
| 19 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 20 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5896` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6183` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 24 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5900` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3154` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11806` |
| 32 | 0.47% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 33 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6521` |
| 34 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 37 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 38 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10127` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1026` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6119` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2587` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15364` |
| 44 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 47 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13146` |
| 50 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10141` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3149` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1031` |
| 54 | 0.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10138` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6192` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 60 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15623` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5897` |
| 6 | 2.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10138` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13052` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6073` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6247` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8733` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13119` |
| 13 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14485` |
| 15 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14494` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6160` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 19 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15661` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10146` |
| 22 | 0.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6131` |
| 24 | 0.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 26 | 0.55% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11813` |
| 29 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

