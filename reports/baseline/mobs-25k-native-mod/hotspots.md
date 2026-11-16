# JVM hotspots (JFR ExecutionSample)

Samples: 2656

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 3 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 7 | 1.96% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 9 | 1.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.98% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.94% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 17 | 0.90% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.56% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.56% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 24 | 0.49% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 604` |
| 26 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.task.ChunkLoadTask.<init>(ChunkTaskScheduler, ServerLevel, int, int, NewChunkHolder, Priority) line: 49` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14107` |
| 30 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 32 | 0.34% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2308` |
| 34 | 0.30% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 39 | 0.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 40 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 41 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.26% | `java.lang.String.length() line: 1583` |
| 43 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.26% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12042` |
| 46 | 0.23% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 47 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 49 | 0.23% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 50 | 0.23% | `java.util.HashMap.getNode(Object) line: 588` |
| 51 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 52 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 53 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1152` |
| 54 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 55 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 57 | 0.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 59 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.23% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 2 | 4.74% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 3 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 5 | 2.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.96% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7504` |
| 11 | 1.47% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9176` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 15 | 0.94% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14095` |
| 17 | 0.87% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.68% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.68% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 24 | 0.68% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12012` |
| 25 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.49% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.49% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 475` |
| 29 | 0.45% | `java.util.HashMap.containsKey(Object) line: 603` |
| 30 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

