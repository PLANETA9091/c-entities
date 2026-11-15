# JVM hotspots (JFR ExecutionSample)

Samples: 2187

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 2 | 4.30% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 1.97% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 7 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.65% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13673` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.01% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 16 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.96% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.69% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 20 | 0.69% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 21 | 0.64% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 22 | 0.55% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 26 | 0.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 28 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.41% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 31 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.41% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11626` |
| 35 | 0.41% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 36 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.37% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 40 | 0.37% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 41 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.32% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 43 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.32% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 45 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 47 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.27% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 49 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13989` |
| 51 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1992` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 53 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 54 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.23% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11643` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1986` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13691` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 8576` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11643` |
| 3 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11576` |
| 5 | 2.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 1.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.97% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.60% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11596` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7127` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5943` |
| 16 | 1.01% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13009` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8760` |
| 21 | 0.64% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.64% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 23 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 25 | 0.59% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.55% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13679` |
| 29 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

