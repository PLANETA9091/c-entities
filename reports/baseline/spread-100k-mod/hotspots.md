# JVM hotspots (JFR ExecutionSample)

Samples: 10310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 2 | 6.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 4 | 4.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.24% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 2.22% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5054` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 11 | 1.67% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 14 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.06% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 1.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 1.01% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7700` |
| 19 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.62% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 22 | 0.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 23 | 0.57% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 24 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 28 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10242` |
| 30 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13390` |
| 34 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13575` |
| 35 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7510` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13529` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7644` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13812` |
| 40 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 41 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11370` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7660` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5328` |
| 46 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13557` |
| 48 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5600` |
| 50 | 0.19% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 51 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 53 | 0.18% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13873` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 57 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11582` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5198` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 10.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 4 | 5.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.56% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5871` |
| 6 | 3.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8656` |
| 7 | 2.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5152` |
| 8 | 1.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7630` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11555` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11488` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5210` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12921` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 16 | 0.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8676` |
| 18 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12930` |
| 20 | 0.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.56% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10249` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 26 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13803` |
| 29 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.39% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |

GC pauses: 0, total 0 ms

