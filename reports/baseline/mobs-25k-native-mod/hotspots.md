# JVM hotspots (JFR ExecutionSample)

Samples: 2531

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 2 | 2.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.77% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 7 | 1.78% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 9 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.54% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12823` |
| 16 | 0.95% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 19 | 0.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.55% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 22 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.47% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 25 | 0.47% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 27 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.43% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 30 | 0.43% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 31 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 32 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 33 | 0.40% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 34 | 0.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 36 | 0.40% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 37 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8318` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 40 | 0.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 41 | 0.36% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 42 | 0.36% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 43 | 0.32% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 44 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1903` |
| 46 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12841` |
| 48 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.24% | `com.viaversion.viaversion.util.MinecraftKey.isValidPath(String) line: 92` |
| 50 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1923` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 53 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 54 | 0.20% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 55 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 56 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 57 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 2 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11201` |
| 5 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 8 | 2.37% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11134` |
| 9 | 2.13% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11154` |
| 11 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.54% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.91% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6939` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11162` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8318` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5836` |
| 21 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.59% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.47% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.47% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 28 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 29 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

