# JVM hotspots (JFR ExecutionSample)

Samples: 11279

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 2 | 7.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 3 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 5 | 3.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 8 | 2.53% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5147` |
| 10 | 1.60% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 14 | 1.13% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.11% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 1.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 1.06% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7478` |
| 20 | 0.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2568` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 23 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 26 | 0.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9888` |
| 28 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 29 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 32 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.35% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2571` |
| 37 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 38 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 40 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12841` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7434` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12782` |
| 44 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 45 | 0.21% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 46 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12823` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7272` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12795` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.18% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 53 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5595` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.slotOf(long) line: 2573` |
| 56 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5284` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |
| 60 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 10.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 4 | 4.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.47% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5836` |
| 6 | 4.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5334` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5258` |
| 9 | 2.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 10 | 1.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11134` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11201` |
| 13 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7420` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 17 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8322` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9895` |
| 20 | 0.55% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5285` |
| 22 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.48% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13011` |
| 25 | 0.47% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2457` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 28 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 29 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

