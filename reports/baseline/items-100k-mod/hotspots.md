# JVM hotspots (JFR ExecutionSample)

Samples: 4573

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 3 | 4.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4777` |
| 6 | 4.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 3.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6284` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4645` |
| 10 | 1.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.79% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4725` |
| 12 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10833` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3414` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6357` |
| 19 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6307` |
| 21 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10845` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6428` |
| 24 | 0.66% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.44% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 27 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6461` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10884` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6414` |
| 31 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10849` |
| 32 | 0.28% | `java.util.ArrayList$Itr.next() line: 1058` |
| 33 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4727` |
| 35 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 36 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 37 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4549` |
| 42 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 46 | 0.17% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3365` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3358` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 410` |
| 50 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 51 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4721` |
| 53 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 409` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3388` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4776` |
| 57 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 58 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 60 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |
| 2 | 6.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 6.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.49% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4678` |
| 7 | 4.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 3.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4542` |
| 9 | 3.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 10 | 2.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 11 | 2.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4676` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4997` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |
| 14 | 1.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4645` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4646` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6414` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3393` |
| 22 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6432` |
| 24 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 29 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.28% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

