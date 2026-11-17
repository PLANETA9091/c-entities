# JVM hotspots (JFR ExecutionSample)

Samples: 3393

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 2 | 4.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.60% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 5 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 6 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 19 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2404` |
| 20 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 21 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 23 | 0.41% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 25 | 0.41% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 26 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 28 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 32 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14374` |
| 34 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 35 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 38 | 0.32% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 39 | 0.32% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 40 | 0.29% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 41 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 42 | 0.27% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 43 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 44 | 0.27% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 45 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 48 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14392` |
| 49 | 0.24% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10983` |
| 51 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 52 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 54 | 0.21% | `java.lang.Class.cast(Object) line: 3491` |
| 55 | 0.21% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 56 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.18% | `com.viaversion.viaversion.util.MinecraftKey.isValidPath(String) line: 90` |
| 58 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 743` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 2 | 4.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12229` |
| 3 | 4.10% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12296` |
| 4 | 3.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 5 | 3.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7650` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9399` |
| 13 | 1.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.62% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.59% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13662` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 24 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.41% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12249` |
| 27 | 0.41% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 28 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14377` |

GC pauses: 0, total 0 ms

