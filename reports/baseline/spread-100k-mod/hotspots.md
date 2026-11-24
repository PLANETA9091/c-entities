# JVM hotspots (JFR ExecutionSample)

Samples: 7519

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9547` |
| 3 | 5.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 4 | 5.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 7 | 1.68% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 9 | 1.56% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6025` |
| 12 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.93% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 0.85% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15432` |
| 17 | 0.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 0.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 21 | 0.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 24 | 0.59% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9243` |
| 27 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.57% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 29 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 30 | 0.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16262` |
| 32 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6765` |
| 34 | 0.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 35 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 39 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10713` |
| 41 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.33% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 43 | 0.31% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9678` |
| 45 | 0.25% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2669` |
| 47 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.24% | `java.util.Collection.stream() line: 748` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16120` |
| 51 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6314` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 56 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.19% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 59 | 0.19% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 60 | 0.19% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.13% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 4 | 4.30% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6026` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 6 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6501` |
| 7 | 2.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6227` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15217` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15226` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13848` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13781` |
| 13 | 0.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16581` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9219` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6269` |
| 19 | 0.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 21 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10841` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.57% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 26 | 0.55% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16247` |
| 29 | 0.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

