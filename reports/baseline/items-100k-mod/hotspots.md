# JVM hotspots (JFR ExecutionSample)

Samples: 5517

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 2 | 8.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 3 | 5.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.12% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7440` |
| 9 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12841` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.92% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 16 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 18 | 0.53% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 20 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 23 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 24 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 25 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 26 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7155` |
| 30 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7396` |
| 32 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.24% | `java.lang.Class.cast(Object) line: 3491` |
| 35 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 38 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8331` |
| 39 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5238` |
| 40 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.18% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 42 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 43 | 0.18% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 44 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 45 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 49 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 50 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5508` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 52 | 0.15% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |
| 53 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12980` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5227` |
| 57 | 0.15% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 58 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 59 | 0.13% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5158` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.46% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 4 | 5.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 5.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8331` |
| 6 | 3.81% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 9 | 1.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13218` |
| 10 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.91% | `java.util.ArrayList.get(int) line: 428` |
| 14 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7374` |
| 15 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 17 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 22 | 0.29% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 23 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.22% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5245` |
| 30 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13237` |

GC pauses: 0, total 0 ms

