# JVM hotspots (JFR ExecutionSample)

Samples: 10480

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 2 | 6.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 3 | 5.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 4 | 4.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.11% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 11 | 1.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7405` |
| 14 | 1.22% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 1.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9814` |
| 24 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 28 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7361` |
| 31 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12923` |
| 34 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11154` |
| 36 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7337` |
| 39 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5477` |
| 41 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 42 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 43 | 0.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 46 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12738` |
| 48 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 49 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 51 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.17% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 54 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.16% | `java.util.Collection.stream() line: 748` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5204` |
| 57 | 0.15% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 10942` |
| 59 | 0.15% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8248` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.85% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 6 | 3.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8228` |
| 8 | 1.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11060` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 12 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 13 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8248` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 17 | 0.70% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 0.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9821` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 25 | 0.48% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6827` |
| 27 | 0.43% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

