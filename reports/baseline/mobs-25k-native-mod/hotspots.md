# JVM hotspots (JFR ExecutionSample)

Samples: 2568

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 2 | 5.49% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.96% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 6 | 2.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 8 | 1.91% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.75% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 16 | 0.82% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11790` |
| 20 | 0.62% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 21 | 0.58% | `java.util.ArrayList$Itr.next() line: 1058` |
| 22 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.51% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.51% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.51% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 27 | 0.51% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 30 | 0.47% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 32 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 34 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 35 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 36 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.35% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 39 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 41 | 0.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 42 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.27% | `java.lang.Class.cast(Object) line: 3491` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 47 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 48 | 0.23% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 49 | 0.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10494` |
| 51 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 53 | 0.19% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 54 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.19% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 57 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.19% | `net.minecraft.resources.DelegatingOps.empty() line: 31` |
| 59 | 0.19% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.19% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 2 | 5.14% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 3 | 4.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 5 | 3.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.71% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8924` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 12 | 1.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7252` |
| 15 | 1.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.82% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13173` |
| 19 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.62% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 22 | 0.58% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.55% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 25 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 28 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.43% | `java.util.HashMap.containsKey(Object) line: 603` |
| 30 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

