# JVM hotspots (JFR ExecutionSample)

Samples: 9559

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 7.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9023` |
| 5 | 3.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 7 | 2.21% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.85% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5904` |
| 11 | 1.16% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14783` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3162` |
| 19 | 0.78% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15459` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 22 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8784` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5908` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6541` |
| 28 | 0.51% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11901` |
| 31 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3157` |
| 32 | 0.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.40% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 34 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 37 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 38 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2626` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2598` |
| 41 | 0.30% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 42 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 43 | 0.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 44 | 0.28% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 13029` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2759` |
| 47 | 0.24% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.23% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 50 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 51 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6212` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8508` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3167` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1167` |
| 57 | 0.17% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.16% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 59 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15718` |
| 3 | 6.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.51% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5905` |
| 5 | 3.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6106` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13214` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14580` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13147` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8758` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14589` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10241` |
| 18 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15756` |
| 20 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15350` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6160` |
| 24 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6276` |
| 26 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15444` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10236` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8762` |

GC pauses: 0, total 0 ms

