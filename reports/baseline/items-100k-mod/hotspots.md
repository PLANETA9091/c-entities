# JVM hotspots (JFR ExecutionSample)

Samples: 7576

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 3 | 4.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 4 | 4.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 5 | 4.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 7 | 2.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.90% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 2303` |
| 9 | 2.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6211` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6209` |
| 13 | 0.88% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 0.81% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 14054` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 16 | 0.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 17 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8687` |
| 19 | 0.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 14056` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8428` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14414` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 29 | 0.36% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 30 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 31 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 32 | 0.25% | `com.viaversion.viaversion.protocol.packet.PacketWrapperImpl.apply(Direction, State, List) line: 453` |
| 33 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 34 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 40 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 41 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.20% | `io.netty.buffer.AbstractByteBuf.writeBytes(ByteBuf, int, int) line: 1111` |
| 43 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 44 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 46 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 49 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 50 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6213` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9608` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6163` |
| 55 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 14055` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10069` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6196` |
| 60 | 0.13% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 5.35% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 3 | 5.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 4 | 4.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9569` |
| 5 | 4.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10078` |
| 6 | 4.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8687` |
| 7 | 4.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 3.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 9 | 3.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 3.00% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 2278` |
| 11 | 2.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.12% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 16 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6220` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14244` |
| 19 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6772` |
| 21 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 23 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14406` |
| 25 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 26 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1431` |
| 27 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.34% | `com.viaversion.viaversion.protocol.ProtocolPipelineImpl.transform(Direction, State, PacketWrapper) line: 110` |
| 29 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 30 | 0.33% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 757` |

GC pauses: 0, total 0 ms

