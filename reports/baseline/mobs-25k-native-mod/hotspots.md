# JVM hotspots (JFR ExecutionSample)

Samples: 1998

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 2 | 4.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 2.10% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.30% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13837` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.70% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 22 | 0.60% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.50% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.50% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 27 | 0.50% | `java.util.HashMap.getNode(Object) line: 586` |
| 28 | 0.45% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.45% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 30 | 0.45% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 31 | 0.40% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11790` |
| 35 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.35% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 38 | 0.35% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 39 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 40 | 0.30% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 42 | 0.30% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 43 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 45 | 0.25% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 46 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 47 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2236` |
| 50 | 0.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.20% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 52 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 53 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.20% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 56 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1622` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.20% | `java.lang.Class.cast(Object) line: 3491` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 446` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 2 | 4.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.90% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 4 | 3.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11740` |
| 5 | 2.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11760` |
| 10 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.30% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8924` |
| 13 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11768` |
| 16 | 1.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7252` |
| 18 | 0.95% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.95% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.60% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13843` |
| 24 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.remove(Entity) line: 456` |
| 25 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13840` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6068` |
| 28 | 0.50% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.50% | `java.util.HashMap.containsKey(Object) line: 603` |
| 30 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

