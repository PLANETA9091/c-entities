# JVM hotspots (JFR ExecutionSample)

Samples: 6911

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 3 | 5.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 4 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13655` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13637` |
| 10 | 1.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7678` |
| 14 | 1.04% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12965` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |
| 16 | 0.87% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 17 | 0.84% | `java.util.HashMap.getNode(Object) line: 586` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5374` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2504` |
| 23 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2508` |
| 25 | 0.61% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13470` |
| 27 | 0.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 28 | 0.54% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13609` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |
| 31 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 33 | 0.46% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 34 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 36 | 0.38% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 37 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.36% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 39 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10294` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8723` |
| 42 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13892` |
| 43 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7982` |
| 45 | 0.29% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 46 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13469` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7700` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13606` |
| 51 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5166` |
| 53 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2503` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11422` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13613` |
| 58 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 60 | 0.20% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 9.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 3 | 8.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 7.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7678` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5923` |
| 9 | 1.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8728` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5262` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13883` |
| 16 | 0.85% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.85% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 18 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11607` |
| 20 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 21 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12982` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12973` |
| 24 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 26 | 0.46% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 27 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11540` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10301` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11560` |

GC pauses: 0, total 0 ms

