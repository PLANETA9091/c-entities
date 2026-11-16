# JVM hotspots (JFR ExecutionSample)

Samples: 10018

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 5.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8152` |
| 3 | 5.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 4.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.42% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 8 | 2.35% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 10 | 1.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 12 | 1.55% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 14 | 1.08% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.05% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.81% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 19 | 0.73% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 21 | 0.60% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 22 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 29 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13856` |
| 33 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 35 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 37 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 38 | 0.30% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10541` |
| 39 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7765` |
| 41 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5844` |
| 43 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 44 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 45 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 47 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14200` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13860` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5524` |
| 52 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 54 | 0.17% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 55 | 0.17% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 56 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 58 | 0.16% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 60 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.38% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 5 | 4.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 6 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 7 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 9 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7885` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 11 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 13 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8975` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 16 | 0.75% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13229` |
| 17 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5290` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14130` |
| 21 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.50% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.47% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 26 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 28 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10548` |
| 30 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

