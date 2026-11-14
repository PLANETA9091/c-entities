# JVM hotspots (JFR ExecutionSample)

Samples: 9499

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 2 | 8.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7189` |
| 3 | 6.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 4 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5297` |
| 8 | 3.19% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.63% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4897` |
| 10 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.51% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.12% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7136` |
| 21 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.54% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5047` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6714` |
| 25 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7952` |
| 28 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 29 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 604` |
| 30 | 0.42% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7092` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.39% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 35 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12424` |
| 38 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12482` |
| 40 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 43 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9538` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7068` |
| 47 | 0.22% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 48 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 49 | 0.18% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 50 | 0.17% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 51 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12436` |
| 54 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 55 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 57 | 0.16% | `java.util.Collection.stream() line: 748` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 59 | 0.16% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7967` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.93% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5533` |
| 6 | 3.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5055` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5008` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10851` |
| 10 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7078` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10784` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5026` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7952` |
| 15 | 0.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7972` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12669` |
| 21 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.54% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.52% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7968` |
| 26 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 28 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 475` |
| 30 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |

GC pauses: 0, total 0 ms

