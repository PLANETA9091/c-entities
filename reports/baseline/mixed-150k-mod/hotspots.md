# JVM hotspots (JFR ExecutionSample)

Samples: 11991

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 3 | 7.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 4 | 4.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 10 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.34% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.26% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.73% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.72% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 20 | 0.69% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.52% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12994` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13040` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 32 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 33 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 35 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1299` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11059` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 38 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 40 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12998` |
| 43 | 0.23% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9931` |
| 45 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 47 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 48 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 49 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 50 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2060` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1933` |
| 54 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 55 | 0.13% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 56 | 0.13% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.13% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 58 | 0.13% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 59 | 0.13% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.28% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 7.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 7 | 4.10% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 8 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 13 | 1.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 17 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8365` |
| 20 | 0.73% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13232` |
| 23 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 28 | 0.52% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms

