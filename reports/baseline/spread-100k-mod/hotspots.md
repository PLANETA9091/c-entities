# JVM hotspots (JFR ExecutionSample)

Samples: 10514

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 2 | 6.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 3 | 5.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 4 | 4.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.72% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.24% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 11 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 14 | 1.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.13% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8254` |
| 17 | 1.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 18 | 0.98% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.51% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 22 | 0.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 23 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.49% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9820` |
| 30 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.33% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5166` |
| 35 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7343` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12744` |
| 38 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 39 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5207` |
| 41 | 0.24% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12929` |
| 43 | 0.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7367` |
| 45 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5477` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8249` |
| 48 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11160` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.17% | `java.util.Collection.stream() line: 748` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7371` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5204` |
| 56 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.16% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 58 | 0.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.26% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.18% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 6 | 3.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8234` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7353` |
| 10 | 1.10% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11133` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11066` |
| 12 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 0.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 15 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 0.58% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.49% | `java.util.HashMap.containsKey(Object) line: 603` |
| 20 | 0.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13134` |
| 21 | 0.47% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7349` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.44% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 26 | 0.42% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9827` |
| 29 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

