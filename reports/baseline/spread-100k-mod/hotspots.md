# JVM hotspots (JFR ExecutionSample)

Samples: 9488

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8105` |
| 3 | 5.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 7 | 3.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.91% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 10 | 2.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 16 | 1.02% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 18 | 0.91% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 20 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 21 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.73% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 23 | 0.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.62% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 25 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.46% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 27 | 0.44% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 28 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 30 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2109` |
| 32 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5477` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5395` |
| 36 | 0.28% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11622` |
| 38 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.25% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 41 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10494` |
| 43 | 0.23% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 44 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5520` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7828` |
| 48 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 51 | 0.20% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2635` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 54 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 55 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 56 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 57 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 59 | 0.15% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 60 | 0.15% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 11.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 5 | 4.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5407` |
| 7 | 3.05% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 8 | 1.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13182` |
| 10 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.19% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 12 | 0.98% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 13 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.75% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 15 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13173` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7838` |
| 19 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8928` |
| 20 | 0.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8924` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5243` |
| 29 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |

GC pauses: 0, total 0 ms

