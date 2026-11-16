# JVM hotspots (JFR ExecutionSample)

Samples: 2776

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 2 | 5.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.67% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.09% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 5 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.66% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.15% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.12% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |
| 17 | 0.72% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 18 | 0.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 22 | 0.58% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 23 | 0.58% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.54% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 30 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 32 | 0.40% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.40% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 36 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 38 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 40 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 41 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.29% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 45 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 46 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 47 | 0.25% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 647` |
| 48 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 49 | 0.25% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 215` |
| 50 | 0.25% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.25% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 53 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 55 | 0.22% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 56 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 57 | 0.22% | `java.lang.String.length() line: 1583` |
| 58 | 0.22% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 59 | 0.22% | `java.util.ArrayList.isEmpty() line: 263` |
| 60 | 0.22% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10541` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 2 | 5.01% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 3 | 4.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.26% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 11 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7299` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8971` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 16 | 0.94% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.79% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.72% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.61% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13890` |
| 24 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.54% | `java.util.HashMap.containsKey(Object) line: 603` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13892` |
| 30 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

