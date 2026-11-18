# JVM hotspots (JFR ExecutionSample)

Samples: 12255

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 2 | 7.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 4 | 4.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8727` |
| 5 | 2.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.56% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 8 | 1.49% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5711` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15031` |
| 11 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 1.04% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14400` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.86% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1033` |
| 20 | 0.82% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5998` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11539` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3009` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6040` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15013` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3005` |
| 29 | 0.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 32 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9860` |
| 35 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 36 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 990` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5715` |
| 39 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 40 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 987` |
| 41 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 43 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 44 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 46 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8509` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3004` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2445` |
| 51 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 12667` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5934` |
| 54 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9874` |
| 56 | 0.23% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6318` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 983` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15024` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 3 | 6.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.72% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 5 | 3.46% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5712` |
| 6 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9871` |
| 7 | 3.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5888` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12852` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12785` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14218` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9879` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14227` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15323` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5975` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 20 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15016` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 25 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 26 | 0.74% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5946` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11546` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |

GC pauses: 0, total 0 ms

