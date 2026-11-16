# JVM hotspots (JFR ExecutionSample)

Samples: 10512

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 5.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 3 | 4.92% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 5 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.34% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.05% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 11 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.23% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 16 | 1.02% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 19 | 0.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.59% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 25 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5647` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 31 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 32 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 37 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |
| 38 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2844` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 43 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10746` |
| 45 | 0.22% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 46 | 0.21% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11874` |
| 48 | 0.21% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 49 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2849` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5779` |
| 53 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.19% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 55 | 0.19% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2288` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5776` |
| 59 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 5 | 4.87% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 6 | 4.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 7 | 2.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.30% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 9 | 1.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 10 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 12 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 15 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9180` |
| 17 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 18 | 0.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13434` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5495` |
| 21 | 0.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.49% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14335` |
| 24 | 0.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.41% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9176` |

GC pauses: 0, total 0 ms

