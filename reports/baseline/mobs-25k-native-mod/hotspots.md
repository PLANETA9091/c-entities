# JVM hotspots (JFR ExecutionSample)

Samples: 3399

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 2 | 4.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.62% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 5 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 7 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 0.82% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 0.76% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 14 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.56% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 18 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 19 | 0.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.53% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 21 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 23 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13557` |
| 25 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 30 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 38 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11517` |
| 39 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 42 | 0.26% | `net.minecraft.world.phys.shapes.VoxelShape.moonrise$isFullBlock() line: 399` |
| 43 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 44 | 0.26% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11516` |
| 46 | 0.24% | `java.util.ArrayList$Itr.next() line: 1058` |
| 47 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 28` |
| 48 | 0.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 389` |
| 49 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10242` |
| 50 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 53 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 55 | 0.21% | `java.lang.Class.cast(Object) line: 3491` |
| 56 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.18% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 58 | 0.18% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 429` |
| 60 | 0.18% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(NoiseChunk, ChunkAccess) line: 465` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11488` |
| 3 | 4.15% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11555` |
| 4 | 3.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7051` |
| 9 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 11 | 1.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5871` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8672` |
| 15 | 0.88% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12921` |
| 18 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.71% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 21 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.53% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 23 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |
| 25 | 0.44% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.38% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |

GC pauses: 0, total 0 ms

