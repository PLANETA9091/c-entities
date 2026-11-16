# JVM hotspots (JFR ExecutionSample)

Samples: 7792

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 3 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 4 | 4.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 4.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 7 | 2.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 12 | 1.48% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.30% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 15 | 1.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 16 | 1.18% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.98% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.92% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 23 | 0.69% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 26 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5777` |
| 28 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14107` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10746` |
| 32 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.42% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 35 | 0.41% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 36 | 0.40% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5738` |
| 38 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5729` |
| 40 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5647` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11874` |
| 43 | 0.32% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 215` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9149` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14061` |
| 46 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2314` |
| 47 | 0.30% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 48 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 49 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.24% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 53 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.21% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 57 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5779` |
| 59 | 0.17% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 60 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 11.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 6.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 5 | 6.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9160` |
| 6 | 2.81% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6320` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5659` |
| 8 | 1.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 9 | 1.48% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13425` |
| 10 | 1.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11992` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12059` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13434` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9180` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 0.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8090` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9176` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5495` |
| 20 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 21 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 22 | 0.59% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10753` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 26 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.49% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

