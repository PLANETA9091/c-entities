# JVM hotspots (JFR ExecutionSample)

Samples: 2570

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12886` |
| 2 | 2.96% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.95% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.56% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12905` |
| 13 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 15 | 1.17% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.86% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 19 | 0.62% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 20 | 0.54% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 21 | 0.47% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 22 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 23 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 26 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 28 | 0.39% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 29 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 33 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8244` |
| 34 | 0.35% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 35 | 0.35% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11110` |
| 36 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.35% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 38 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.31% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 40 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.31% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 42 | 0.31% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 43 | 0.31% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 44 | 0.31% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 45 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 46 | 0.27% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 47 | 0.27% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 48 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.23% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.23% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 54 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 56 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9814` |
| 60 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 2 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11060` |
| 6 | 2.53% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 7 | 2.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.95% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5740` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11080` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.17% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6827` |
| 18 | 0.86% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8244` |
| 21 | 0.62% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 22 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11088` |
| 24 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.43% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.39% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

