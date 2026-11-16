# JVM hotspots (JFR ExecutionSample)

Samples: 10413

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 2 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8105` |
| 3 | 5.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 4 | 4.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 1.72% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.66% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 14 | 1.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.23% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 0.84% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.56% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 23 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 25 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.46% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 28 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 34 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 36 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 38 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13809` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 43 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 44 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2635` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5797` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2631` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 49 | 0.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 50 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2083` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7718` |
| 54 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.18% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 56 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5395` |
| 58 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5516` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 5 | 4.65% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 6 | 3.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 7 | 1.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7838` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5407` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 12 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8928` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13173` |
| 15 | 0.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 17 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13182` |
| 19 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 20 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5243` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14083` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.47% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 26 | 0.45% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.43% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 29 | 0.42% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 30 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

