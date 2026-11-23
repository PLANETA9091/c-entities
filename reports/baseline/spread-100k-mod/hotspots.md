# JVM hotspots (JFR ExecutionSample)

Samples: 9930

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 4.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9450` |
| 4 | 3.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.65% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.29% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 8 | 1.66% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6706` |
| 10 | 1.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 11 | 1.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 13 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5967` |
| 20 | 0.67% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 22 | 0.63% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16013` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 30 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.44% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.43% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5971` |
| 33 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 35 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 37 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13781` |
| 40 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 41 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2610` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6715` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 45 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 47 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 49 | 0.23% | `java.lang.Double.isFinite(double) line: 1023` |
| 50 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6705` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10616` |
| 53 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.20% | `java.util.ArrayList$Itr.next() line: 1058` |
| 55 | 0.20% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 57 | 0.19% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 59 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16272` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5968` |
| 5 | 2.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6441` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13684` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9137` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13751` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6169` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15120` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16332` |
| 14 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15129` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10744` |
| 18 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.70% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6211` |
| 23 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 25 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15998` |
| 27 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

