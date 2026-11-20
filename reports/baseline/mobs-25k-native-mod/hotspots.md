# JVM hotspots (JFR ExecutionSample)

Samples: 3550

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.87% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 3 | 2.79% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 4 | 2.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 5 | 2.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 6 | 2.14% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 7 | 1.97% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.69% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 14 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 1.01% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 0.85% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 0.82% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 22 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 23 | 0.68% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.compareAndSet(VarHandle, Object, int, int) line: 178` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 25 | 0.62% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 27 | 0.45% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13042` |
| 29 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 32 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 33 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 35 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 36 | 0.39% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 37 | 0.39% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 38 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 39 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 40 | 0.37% | `java.util.stream.ReferencePipeline$2$1.accept(Object) line: 196` |
| 41 | 0.34% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.34% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2589` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 45 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 46 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 47 | 0.31% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 48 | 0.31% | `java.util.ArrayList$Itr.next() line: 1058` |
| 49 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.28% | `java.util.ImmutableCollections.listCopy(Collection) line: 191` |
| 51 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 52 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 53 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 55 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.getBukkitEntity() line: 157` |
| 58 | 0.25% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 59 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 183` |
| 60 | 0.25% | `java.util.ImmutableCollections.listCopy(Collection) line: 188` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 3.32% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 3 | 3.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13059` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12992` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9569` |
| 10 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 11 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 13 | 1.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.18% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8030` |
| 20 | 1.01% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14425` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13012` |
| 24 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 25 | 0.82% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10082` |
| 27 | 0.73% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 441` |
| 29 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 30 | 0.68% | `io.netty.util.internal.RefCnt$VarHandleRefCnt.release0(RefCnt, int) line: 371` |

GC pauses: 0, total 0 ms

