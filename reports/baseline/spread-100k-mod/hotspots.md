# JVM hotspots (JFR ExecutionSample)

Samples: 6397

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 4.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 4 | 3.83% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9466` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6253` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9162` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16029` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 11 | 1.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.89% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15351` |
| 18 | 0.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6714` |
| 20 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9134` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5968` |
| 24 | 0.64% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 25 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5964` |
| 29 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 30 | 0.58% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 31 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 32 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 34 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 36 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6705` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6057` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 41 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6208` |
| 43 | 0.33% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 44 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2647` |
| 46 | 0.30% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 47 | 0.30% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 48 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 49 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 50 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 51 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 52 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 55 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 56 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 57 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.23% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 3 | 5.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 5 | 3.10% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6208` |
| 6 | 2.00% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5965` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6166` |
| 8 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6440` |
| 10 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.05% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5966` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10760` |
| 15 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 16 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15145` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16348` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 20 | 0.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13767` |
| 23 | 0.72% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5967` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13720` |
| 25 | 0.64% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9134` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13700` |
| 29 | 0.59% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15136` |

GC pauses: 0, total 0 ms

