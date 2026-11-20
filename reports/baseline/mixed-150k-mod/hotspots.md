# JVM hotspots (JFR ExecutionSample)

Samples: 17343

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 3 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 5 | 3.91% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 6 | 3.78% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.16% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 13 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.95% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 0.93% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3121` |
| 19 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 20 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 21 | 0.66% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8673` |
| 23 | 0.61% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.54% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6211` |
| 26 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 28 | 0.50% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 29 | 0.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6209` |
| 32 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8428` |
| 35 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 36 | 0.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 37 | 0.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11746` |
| 39 | 0.35% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3125` |
| 42 | 0.33% | `java.util.concurrent.atomic.AtomicLongFieldUpdater$CASUpdater.compareAndSet(Object, long, long) line: 444` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 44 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 45 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8700` |
| 46 | 0.31% | `jdk.internal.foreign.MemorySessionImpl.checkValidStateRaw() line: 203` |
| 47 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 49 | 0.25% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1130` |
| 51 | 0.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 52 | 0.24% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 56 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 57 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.21% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 59 | 0.21% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 805` |
| 60 | 0.20% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.93% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 5 | 5.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9569` |
| 6 | 3.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 7 | 3.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 8 | 2.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 2.41% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 2.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10078` |
| 12 | 1.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 13 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8679` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14425` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13059` |
| 18 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 19 | 0.84% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12992` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10086` |
| 23 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 441` |
| 26 | 0.61% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14434` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14244` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5938` |

GC pauses: 0, total 0 ms

