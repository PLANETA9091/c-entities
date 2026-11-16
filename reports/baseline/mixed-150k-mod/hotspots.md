# JVM hotspots (JFR ExecutionSample)

Samples: 11250

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 8.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 3 | 6.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 5.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 9 | 1.57% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 10 | 1.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.30% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2840` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 16 | 0.94% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.90% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 19 | 0.74% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 29 | 0.57% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2844` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14107` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 33 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5729` |
| 35 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 36 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14065` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 38 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 41 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 44 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2308` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10746` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14405` |
| 50 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2288` |
| 52 | 0.19% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7898` |
| 54 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5689` |
| 56 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 57 | 0.16% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9180` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6049` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 11.34% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 11.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 8.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 5 | 4.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 6 | 4.62% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 7 | 2.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 9 | 1.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 12 | 1.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14335` |
| 17 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9180` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13434` |
| 23 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 25 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.57% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5495` |
| 28 | 0.44% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 30 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

