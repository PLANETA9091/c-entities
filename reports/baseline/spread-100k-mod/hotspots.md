# JVM hotspots (JFR ExecutionSample)

Samples: 14650

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9987` |
| 3 | 3.67% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 3.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8894` |
| 5 | 3.20% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 9 | 1.67% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14327` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8630` |
| 13 | 1.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.98% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 16 | 0.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6153` |
| 18 | 0.94% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.87% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.61% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.51% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.48% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 27 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 28 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.43% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 32 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6159` |
| 33 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8360` |
| 34 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 36 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11656` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8608` |
| 41 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 44 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 45 | 0.29% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9977` |
| 47 | 0.28% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 48 | 0.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 49 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 50 | 0.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 51 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6157` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.21% | `net.minecraft.world.entity.LivingEntity.getYHeadRot() line: 4106` |
| 55 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6433` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3082` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3838` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9991` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 5.88% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 4 | 5.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9479` |
| 6 | 3.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 3.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 8 | 2.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9988` |
| 9 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 1.36% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 12 | 1.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12902` |
| 14 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1282` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8612` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12969` |
| 17 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 18 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14335` |
| 21 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8608` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14344` |
| 24 | 0.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9996` |
| 26 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14154` |
| 28 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 29 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 441` |

GC pauses: 0, total 0 ms

