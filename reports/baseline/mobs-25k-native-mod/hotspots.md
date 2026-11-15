# JVM hotspots (JFR ExecutionSample)

Samples: 2624

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 2 | 4.92% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 4 | 2.93% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13061` |
| 6 | 2.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 13 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.91% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.91% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13751` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11686` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 23 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13733` |
| 24 | 0.50% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 26 | 0.46% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 27 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 28 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 31 | 0.42% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 32 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.38% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 35 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11706` |
| 37 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.30% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 39 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 40 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 41 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2046` |
| 43 | 0.30% | `java.util.ArrayList$Itr.next() line: 1058` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1524` |
| 45 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.27% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 47 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.27% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2243` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 50 | 0.27% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 51 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2052` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 53 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 54 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 55 | 0.23% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 57 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 58 | 0.23% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 59 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.removeEntity(Entity, int) line: 314` |
| 60 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 2 | 4.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11703` |
| 3 | 3.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.58% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11636` |
| 5 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6003` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7187` |
| 11 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 13 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8820` |
| 16 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13069` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.65% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.57% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 23 | 0.50% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.50% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 25 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 26 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.42% | `java.util.ArrayList.grow(int) line: 238` |
| 28 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13739` |
| 30 | 0.38% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |

GC pauses: 0, total 0 ms

