# JVM hotspots (JFR ExecutionSample)

Samples: 4421

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.33% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9977` |
| 6 | 2.40% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.92% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 13 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 14 | 1.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.79% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.68% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 22 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.48% | `java.util.concurrent.atomic.AtomicLongFieldUpdater$CASUpdater.compareAndSet(Object, long, long) line: 444` |
| 26 | 0.48% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 27 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2546` |
| 30 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 31 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 32 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 34 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.32% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 36 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 37 | 0.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 38 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.27% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12952` |
| 42 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 43 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 44 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 47 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 52 | 0.23% | `java.util.stream.ReferencePipeline$2$1.accept(Object) line: 196` |
| 53 | 0.23% | `java.util.concurrent.atomic.AtomicReferenceArray.lazySet(int, Object) line: 126` |
| 54 | 0.23% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 55 | 0.20% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 56 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 57 | 0.20% | `java.util.ArrayList.forEach(Consumer) line: 1604` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 59 | 0.20% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 805` |
| 60 | 0.20% | `java.util.ArrayDeque.circularClear(Object[], int, int) line: 1049` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 2 | 3.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12969` |
| 3 | 3.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 2.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12902` |
| 6 | 2.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 7 | 2.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9479` |
| 9 | 2.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 2.08% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 16 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 17 | 1.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7962` |
| 19 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14335` |
| 22 | 0.77% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 441` |
| 24 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.68% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 757` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14154` |
| 28 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9992` |
| 30 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |

GC pauses: 0, total 0 ms

