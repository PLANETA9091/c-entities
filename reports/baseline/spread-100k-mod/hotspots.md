# JVM hotspots (JFR ExecutionSample)

Samples: 8863

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7819` |
| 2 | 7.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 3 | 6.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 4 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.79% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5291` |
| 9 | 2.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 2.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5031` |
| 12 | 1.71% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 13 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.93% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5295` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7617` |
| 21 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.67% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 23 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.47% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5257` |
| 26 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7613` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5297` |
| 28 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 29 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7595` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13497` |
| 34 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7479` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13312` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 39 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11305` |
| 42 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5175` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10177` |
| 46 | 0.19% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5569` |
| 48 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2485` |
| 50 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 52 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.17% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 54 | 0.17% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 55 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1154` |
| 57 | 0.16% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8611` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2086` |
| 60 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 8.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 6.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8591` |
| 6 | 3.69% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5840` |
| 7 | 1.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5129` |
| 9 | 1.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5187` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7599` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11490` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11423` |
| 13 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12849` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8611` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12858` |
| 17 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.64% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7595` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13702` |
| 23 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8607` |
| 25 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 26 | 0.43% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13485` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.38% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

