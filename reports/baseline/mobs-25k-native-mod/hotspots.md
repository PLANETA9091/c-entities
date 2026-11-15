# JVM hotspots (JFR ExecutionSample)

Samples: 2730

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.93% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13460` |
| 3 | 2.82% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 5 | 2.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 8 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13479` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.81% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.73% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 20 | 0.62% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 21 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 22 | 0.55% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 23 | 0.48% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 24 | 0.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 25 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 29 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 30 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13497` |
| 33 | 0.40% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 34 | 0.40% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 35 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 38 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13702` |
| 39 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 41 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11473` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1953` |
| 44 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 45 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.29% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 50 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.26% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.26% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 53 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.22% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 55 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.22% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11470` |
| 58 | 0.22% | `java.util.ArrayList.isEmpty() line: 263` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 8423` |
| 60 | 0.18% | `java.util.stream.ReferencePipeline$3$1.accept(Object) line: 214` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13702` |
| 2 | 5.57% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11490` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11423` |
| 4 | 2.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 5 | 2.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 8 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7020` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5840` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12849` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8607` |
| 16 | 0.84% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 17 | 0.81% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 18 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.73% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 20 | 0.66% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11443` |
| 25 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13487` |
| 27 | 0.48% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 28 | 0.48% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13485` |
| 30 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

