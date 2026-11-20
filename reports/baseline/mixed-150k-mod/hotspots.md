# JVM hotspots (JFR ExecutionSample)

Samples: 18593

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8844` |
| 4 | 3.96% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 5 | 3.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 12 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6153` |
| 14 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3078` |
| 16 | 1.03% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8630` |
| 18 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.84% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 21 | 0.75% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 22 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 23 | 0.67% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.60% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8360` |
| 27 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 29 | 0.52% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6159` |
| 32 | 0.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 33 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3838` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6120` |
| 36 | 0.39% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 37 | 0.38% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 38 | 0.36% | `java.util.concurrent.atomic.AtomicLongFieldUpdater$CASUpdater.compareAndSet(Object, long, long) line: 444` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6157` |
| 40 | 0.34% | `jdk.internal.misc.ScopedMemoryAccess.putByteInternal(MemorySessionImpl, Object, long, byte) line: 1121` |
| 41 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 42 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 43 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3082` |
| 45 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 46 | 0.32% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11656` |
| 47 | 0.30% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8626` |
| 49 | 0.27% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 50 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 51 | 0.27% | `java.util.ImmutableCollections.listCopy(Collection) line: 188` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 53 | 0.24% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 54 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2515` |
| 57 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 59 | 0.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.88% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 5 | 5.36% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9479` |
| 6 | 3.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 7 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 8 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9988` |
| 9 | 2.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 2.00% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8608` |
| 12 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 13 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 14 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 15 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12969` |
| 17 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 18 | 0.89% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14335` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12902` |
| 21 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8612` |
| 24 | 0.66% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 441` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14154` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14344` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9996` |
| 29 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 30 | 0.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |

GC pauses: 0, total 0 ms

