# JVM hotspots (JFR ExecutionSample)

Samples: 4241

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 4.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 6 | 4.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 8 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 9 | 2.71% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 12 | 1.84% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.94% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 18 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 21 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 27 | 0.47% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 30 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 33 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 761` |
| 34 | 0.28% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 35 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2808` |
| 38 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 43 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 49 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3975` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 54 | 0.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4126` |
| 56 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 57 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 58 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 59 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 6.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 5.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 6 | 4.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 7 | 3.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 9 | 2.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 11 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 12 | 1.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 14 | 1.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 22 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 24 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 27 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.28% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.<init>() line: 90` |
| 29 | 0.26% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

