# JVM hotspots (JFR ExecutionSample)

Samples: 10377

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 7.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 6.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 4 | 5.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 6 | 2.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.01% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 17 | 1.08% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 18 | 1.07% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.85% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 21 | 0.74% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.63% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.60% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 26 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7908` |
| 29 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 30 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 31 | 0.43% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 32 | 0.40% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 33 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13813` |
| 35 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 37 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |
| 39 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7828` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11622` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13809` |
| 42 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 44 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5477` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14153` |
| 47 | 0.26% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10494` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7646` |
| 51 | 0.22% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 52 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2109` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2108` |
| 57 | 0.19% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 58 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 59 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 9.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 9.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 4 | 5.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8908` |
| 5 | 4.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5407` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 9 | 1.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 10 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13182` |
| 12 | 1.15% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8928` |
| 14 | 1.10% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.07% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 17 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13173` |
| 20 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.63% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 22 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5243` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8924` |
| 26 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11760` |
| 28 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.43% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14083` |

GC pauses: 0, total 0 ms

