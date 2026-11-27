# JVM hotspots (JFR ExecutionSample)

Samples: 7636

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 2 | 5.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9583` |
| 3 | 4.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.61% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 3.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 9 | 1.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13960` |
| 11 | 1.52% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6034` |
| 15 | 0.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.71% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9279` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16590` |
| 21 | 0.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 23 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3245` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16405` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 29 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6038` |
| 32 | 0.45% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6784` |
| 37 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13903` |
| 40 | 0.33% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6774` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 43 | 0.31% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10847` |
| 45 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.26% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2675` |
| 50 | 0.25% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 51 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9714` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 55 | 0.24% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 56 | 0.21% | `java.util.Collection.stream() line: 748` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9101` |
| 58 | 0.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.20% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 3.50% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6035` |
| 4 | 3.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16575` |
| 5 | 3.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 6 | 2.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6510` |
| 8 | 2.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6236` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15299` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15308` |
| 13 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9255` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13826` |
| 17 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6278` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10877` |
| 24 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9251` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16583` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 7244` |
| 28 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.52% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |

GC pauses: 0, total 0 ms

