# JVM hotspots (JFR ExecutionSample)

Samples: 2568

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 2 | 5.69% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.61% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 1.99% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10725` |
| 9 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.40% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 12 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.93% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.93% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.82% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2700` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 23 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 24 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 27 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 28 | 0.43% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 31 | 0.39% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.39% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.39% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 34 | 0.39% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 35 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 37 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 38 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.31% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 40 | 0.31% | `java.lang.Class.cast(Object) line: 3491` |
| 41 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 42 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.27% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 46 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 47 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.27% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16262` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 51 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 53 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 741` |
| 54 | 0.23% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 56 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 57 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 58 | 0.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.23% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15432` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 2 | 5.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13848` |
| 3 | 4.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13781` |
| 4 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 1.99% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 8 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.71% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 10 | 1.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16247` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10820` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15217` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8477` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2207` |
| 19 | 0.66% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 23 | 0.55% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13801` |
| 26 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 27 | 0.55% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 29 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.47% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |

GC pauses: 0, total 0 ms

