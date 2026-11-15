# JVM hotspots (JFR ExecutionSample)

Samples: 10368

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 2 | 6.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 3 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 4 | 4.09% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.19% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.88% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 11 | 1.41% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7436` |
| 14 | 1.25% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.19% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 1.01% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 18 | 0.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 21 | 0.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.43% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9895` |
| 31 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 32 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 35 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 36 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 37 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13004` |
| 38 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 39 | 0.29% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 43 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.26% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 48 | 0.23% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 49 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 50 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 51 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 52 | 0.18% | `java.util.Collection.stream() line: 748` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12958` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8324` |
| 55 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12819` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5238` |
| 58 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5063` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.04% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 7 | 3.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 8 | 1.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 1.35% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11208` |
| 10 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 12 | 1.03% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11141` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8329` |
| 16 | 0.71% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 17 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13209` |
| 22 | 0.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9902` |
| 24 | 0.46% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.44% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.44% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.43% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7374` |
| 29 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

