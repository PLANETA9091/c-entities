# JVM hotspots (JFR ExecutionSample)

Samples: 12109

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 2 | 6.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10077` |
| 5 | 4.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8984` |
| 7 | 3.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 8 | 2.36% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6211` |
| 11 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8700` |
| 14 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 1.16% | `java.util.HashMap.getNode(Object) line: 586` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.78% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6209` |
| 21 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 22 | 0.71% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 26 | 0.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 27 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11746` |
| 29 | 0.51% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 30 | 0.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 31 | 0.46% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9608` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9061` |
| 35 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 36 | 0.39% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.37% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 39 | 0.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 40 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9620` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6154` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6485` |
| 45 | 0.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 46 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8786` |
| 48 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 50 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1288` |
| 51 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.22% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8673` |
| 54 | 0.21% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1130` |
| 56 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6090` |
| 58 | 0.19% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 59 | 0.18% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8428` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 7.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 3 | 7.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.44% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 7 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10078` |
| 8 | 2.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9569` |
| 9 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1282` |
| 11 | 1.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 12 | 1.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 13 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 14 | 1.12% | `java.util.HashMap.containsKey(Object) line: 603` |
| 15 | 1.07% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9622` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14434` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12992` |
| 19 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13059` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10086` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11753` |
| 26 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14425` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8683` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5938` |
| 30 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |

GC pauses: 0, total 0 ms

