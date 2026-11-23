# JVM hotspots (JFR ExecutionSample)

Samples: 10066

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 2 | 6.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 5.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9416` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 7 | 1.74% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10761` |
| 9 | 1.33% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9162` |
| 11 | 1.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 12 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6705` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15351` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16029` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5964` |
| 17 | 0.97% | `java.util.HashMap.getNode(Object) line: 586` |
| 18 | 0.93% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 22 | 0.70% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 26 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.48% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5968` |
| 31 | 0.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 32 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3172` |
| 34 | 0.46% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10633` |
| 36 | 0.37% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 37 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2774` |
| 40 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9134` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 43 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10730` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 45 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2613` |
| 47 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6704` |
| 49 | 0.23% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 51 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2638` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6714` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6233` |
| 56 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 59 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 60 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 3 | 6.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.51% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5965` |
| 6 | 3.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6166` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13767` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6440` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13700` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9134` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15136` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16348` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15920` |
| 19 | 0.91% | `java.util.HashMap.containsKey(Object) line: 603` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15145` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6208` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16014` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 27 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2647` |
| 29 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 30 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

