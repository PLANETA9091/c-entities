# JVM hotspots (JFR ExecutionSample)

Samples: 6566

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 6.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8844` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 4 | 4.04% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 5 | 3.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.27% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 2260` |
| 7 | 3.06% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 3.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 9 | 2.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6157` |
| 10 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 2.47% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 12 | 2.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6153` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6159` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9518` |
| 15 | 0.91% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 16 | 0.81% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14324` |
| 19 | 0.73% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 20 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 13964` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9979` |
| 23 | 0.50% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.compareAndSet(VarHandle, Object, int, int) line: 178` |
| 24 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8360` |
| 25 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 26 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 13966` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8616` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9530` |
| 30 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 31 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 32 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 33 | 0.34% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 34 | 0.34% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1088` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 36 | 0.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.32% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1087` |
| 38 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 13965` |
| 40 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6120` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9988` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8894` |
| 44 | 0.24% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 805` |
| 45 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.23% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 47 | 0.23% | `net.minecraft.world.entity.Entity.getZ() line: 5140` |
| 48 | 0.23% | `net.minecraft.world.entity.Entity.getUUID() line: 4506` |
| 49 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8626` |
| 51 | 0.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 52 | 0.20% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 54 | 0.18% | `java.lang.Double.isFinite(double) line: 1023` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6111` |
| 56 | 0.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 57 | 0.18% | `java.util.UUID.getMostSignificantBits() line: 312` |
| 58 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 59 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9988` |
| 3 | 7.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9479` |
| 4 | 5.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 5.42% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 6 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 7 | 3.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8616` |
| 8 | 3.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 3.49% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 2235` |
| 10 | 3.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 2.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 2.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1431` |
| 16 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14154` |
| 19 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14316` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 23 | 0.59% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 24 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 25 | 0.50% | `io.netty.util.internal.RefCnt$VarHandleRefCnt.release0(RefCnt, int) line: 371` |
| 26 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1282` |
| 28 | 0.35% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 757` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9478` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9532` |

GC pauses: 0, total 0 ms

