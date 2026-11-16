# JVM hotspots (JFR ExecutionSample)

Samples: 13354

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 8.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 8.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 4 | 5.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 6 | 3.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7955` |
| 14 | 1.27% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 16 | 1.15% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.91% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 22 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.69% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13856` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2109` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13860` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 32 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8229` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 36 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 38 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.40% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 40 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 43 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 45 | 0.33% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10541` |
| 46 | 0.26% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 47 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.24% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2644` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14200` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5524` |
| 53 | 0.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 54 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntityStatus(Entity) line: 401` |
| 55 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7969` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8970` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13895` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5442` |
| 60 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 12.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 10.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 8.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 5 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 8 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13229` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 14 | 1.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 17 | 1.01% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14130` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8975` |
| 21 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 22 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7907` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8971` |
| 25 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5290` |

GC pauses: 0, total 0 ms

