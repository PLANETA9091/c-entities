# JVM hotspots (JFR ExecutionSample)

Samples: 2521

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12320` |
| 3 | 2.50% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 1.94% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 8 | 1.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.95% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12339` |
| 17 | 0.71% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.67% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 21 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.56% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 23 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 24 | 0.52% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.52% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.48% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 28 | 0.48% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.48% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 30 | 0.44% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.44% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10709` |
| 33 | 0.44% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 34 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 36 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 37 | 0.40% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 38 | 0.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 39 | 0.36% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 41 | 0.36% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12544` |
| 43 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 44 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12357` |
| 45 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 46 | 0.28% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 47 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.24% | `java.util.ArrayList.grow(int) line: 238` |
| 50 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.24% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 53 | 0.24% | `net.minecraft.world.level.levelgen.material.MaterialRuleContext$LazyYCondition.test() line: 333` |
| 54 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9413` |
| 56 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.24% | `java.util.ArrayList.isEmpty() line: 263` |
| 60 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12544` |
| 2 | 4.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10726` |
| 3 | 3.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10659` |
| 4 | 2.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.94% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6494` |
| 11 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 0.99% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10679` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7843` |
| 17 | 0.75% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.71% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12345` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.56% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.56% | `java.util.HashMap.containsKey(Object) line: 603` |
| 25 | 0.56% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 26 | 0.56% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 27 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.52% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.52% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10687` |

GC pauses: 0, total 0 ms

