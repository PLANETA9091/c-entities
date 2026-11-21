# JVM hotspots (JFR ExecutionSample)

Samples: 9122

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 5.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 3 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 4 | 3.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.77% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.65% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 9 | 1.67% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.44% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.32% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 17 | 0.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |
| 19 | 0.72% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 20 | 0.70% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 21 | 0.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 25 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 26 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5928` |
| 29 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 31 | 0.44% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 32 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15556` |
| 33 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 34 | 0.36% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 35 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.35% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11984` |
| 37 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 38 | 0.34% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 39 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 40 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 13112` |
| 43 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 45 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 46 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 48 | 0.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2599` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3168` |
| 51 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6609` |
| 52 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2760` |
| 54 | 0.21% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 56 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13324` |
| 59 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 60 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.45% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 4.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 4 | 2.86% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5925` |
| 5 | 2.47% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 6 | 2.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6126` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14663` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13230` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13297` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14672` |
| 15 | 0.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15853` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10324` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10319` |
| 21 | 0.66% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 22 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 25 | 0.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 26 | 0.57% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 27 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

