# JVM hotspots (JFR ExecutionSample)

Samples: 9572

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 2 | 7.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 3 | 6.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 4 | 4.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.58% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.16% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.99% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.76% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 19 | 0.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 23 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.44% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 27 | 0.44% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7440` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13026` |
| 30 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9917` |
| 33 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 34 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12841` |
| 37 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 39 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 40 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 43 | 0.23% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 44 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11045` |
| 46 | 0.21% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 47 | 0.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 48 | 0.20% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 49 | 0.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 50 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 52 | 0.18% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12980` |
| 54 | 0.17% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 55 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 56 | 0.15% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.23% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 6 | 3.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8331` |
| 7 | 3.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11230` |
| 10 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11163` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13231` |
| 16 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8351` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 22 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.46% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.45% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.44% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8347` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.43% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

