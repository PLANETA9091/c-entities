# JVM hotspots (JFR ExecutionSample)

Samples: 10973

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 2 | 6.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 3 | 5.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 4 | 4.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.49% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 9 | 2.35% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4904` |
| 10 | 2.12% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.77% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7195` |
| 14 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 1.05% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5070` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9604` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.36% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 34 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12490` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10732` |
| 37 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5329` |
| 39 | 0.25% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 41 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7127` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12548` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8007` |
| 45 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4986` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8033` |
| 49 | 0.17% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12502` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 53 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |
| 55 | 0.16% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 56 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.15% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 58 | 0.15% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 60 | 0.15% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.90% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5569` |
| 5 | 4.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5078` |
| 8 | 2.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5015` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7137` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10917` |
| 12 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10850` |
| 14 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8038` |
| 17 | 0.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.69% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5039` |
| 20 | 0.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9611` |
| 23 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.45% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8034` |
| 27 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 28 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

