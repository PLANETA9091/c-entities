# JVM hotspots (JFR ExecutionSample)

Samples: 4331

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 8.57% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 3 | 6.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 4 | 4.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 8 | 2.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 10 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6553` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 13 | 1.50% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6911` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12242` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6915` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4891` |
| 22 | 0.48% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6790` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 26 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6895` |
| 28 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 34 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 37 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6741` |
| 38 | 0.18% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 39 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.16% | `java.lang.Class.cast(Object) line: 3491` |
| 42 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.16% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 44 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 45 | 0.14% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 46 | 0.14% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 47 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.14% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 49 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 50 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.14% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 53 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 54 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.12% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.12% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.12% | `net.minecraft.server.level.ChunkMap.addEntity(Entity) line: 1017` |
| 58 | 0.12% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.12% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 60 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7101` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 3 | 8.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 4 | 4.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 3.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4870` |
| 7 | 3.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6840` |
| 9 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4894` |
| 10 | 2.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 2.08% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12474` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 15 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.55% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6915` |
| 21 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12493` |
| 29 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12436` |
| 30 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6969` |

GC pauses: 0, total 0 ms

