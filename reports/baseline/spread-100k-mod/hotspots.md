# JVM hotspots (JFR ExecutionSample)

Samples: 10899

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 2 | 6.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7973` |
| 3 | 4.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 4 | 3.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.50% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.21% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5394` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7731` |
| 12 | 1.46% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.28% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.16% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 1.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5270` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5398` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.50% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 27 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 29 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13691` |
| 31 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10330` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13506` |
| 34 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7593` |
| 35 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2518` |
| 37 | 0.29% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 38 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 39 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7727` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7709` |
| 42 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13673` |
| 44 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5400` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11458` |
| 46 | 0.23% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8759` |
| 48 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13645` |
| 50 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5402` |
| 53 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 54 | 0.18% | `java.util.Collection.stream() line: 748` |
| 55 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2514` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13642` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8744` |
| 59 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 60 | 0.17% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 5 | 4.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8744` |
| 6 | 3.86% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5943` |
| 7 | 2.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11576` |
| 9 | 1.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11643` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7713` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13009` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13018` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7709` |
| 15 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5282` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8764` |
| 17 | 0.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 23 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5118` |
| 25 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10337` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13919` |
| 28 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.39% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |

GC pauses: 0, total 0 ms

