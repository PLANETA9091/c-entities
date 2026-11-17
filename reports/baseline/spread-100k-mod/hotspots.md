# JVM hotspots (JFR ExecutionSample)

Samples: 8810

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 2 | 8.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8775` |
| 5 | 2.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.58% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 7 | 1.51% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |
| 9 | 1.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6038` |
| 11 | 1.27% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6042` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5709` |
| 17 | 0.75% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9598` |
| 19 | 0.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5932` |
| 21 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 22 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14039` |
| 24 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8507` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3007` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5713` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6316` |
| 30 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5996` |
| 31 | 0.49% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1033` |
| 32 | 0.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 34 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 35 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14652` |
| 36 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14670` |
| 37 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 40 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8511` |
| 43 | 0.34% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2440` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11178` |
| 46 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.30% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 987` |
| 49 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.standQuietStamp(ArmorStand, int) line: 5576` |
| 51 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.26% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1030` |
| 53 | 0.25% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9567` |
| 55 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9578` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3002` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 59 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 60 | 0.19% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.99% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 4 | 4.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9578` |
| 5 | 2.95% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 6 | 2.79% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5710` |
| 7 | 2.67% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |
| 8 | 1.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5886` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12491` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12424` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8493` |
| 13 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13857` |
| 15 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13866` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5944` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14962` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5711` |
| 26 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14655` |
| 28 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5712` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6053` |

GC pauses: 0, total 0 ms

