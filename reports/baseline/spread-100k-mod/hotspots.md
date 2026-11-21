# JVM hotspots (JFR ExecutionSample)

Samples: 9019

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 3 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 4 | 4.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.17% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 8 | 1.72% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.30% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 14 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 18 | 0.85% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 19 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.58% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 24 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 30 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 32 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.39% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 34 | 0.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6609` |
| 35 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 36 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10344` |
| 38 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.31% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 40 | 0.31% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 41 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5928` |
| 43 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1034` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 48 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13411` |
| 49 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2760` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6599` |
| 52 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 53 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 56 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 57 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2596` |
| 59 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.03% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 4 | 3.18% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5925` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 6 | 2.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 8 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |
| 9 | 1.53% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13314` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13381` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14750` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6126` |
| 14 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10374` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14759` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15940` |
| 19 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 21 | 0.71% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.65% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 24 | 0.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 25 | 0.53% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 30 | 0.47% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

