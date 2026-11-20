# JVM hotspots (JFR ExecutionSample)

Samples: 2373

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 2 | 3.20% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15563` |
| 6 | 1.81% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 7 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 12 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 13 | 1.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.18% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |
| 16 | 0.72% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 18 | 0.59% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 19 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 20 | 0.59% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 21 | 0.59% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 23 | 0.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 24 | 0.51% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 25 | 0.51% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2589` |
| 27 | 0.46% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 28 | 0.46% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13042` |
| 30 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.removeEntity(Entity, int) line: 314` |
| 32 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 36 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.38% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 38 | 0.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 39 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 40 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.29% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11746` |
| 47 | 0.29% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14628` |
| 49 | 0.29% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 50 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 51 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 52 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 55 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 56 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 57 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 60 | 0.21% | `net.minecraft.server.level.ChunkMap.removeEntity(Entity) line: 1032` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15563` |
| 2 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.03% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13059` |
| 4 | 2.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 6 | 2.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.11% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12992` |
| 8 | 1.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.69% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13012` |
| 10 | 1.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8030` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13020` |
| 14 | 1.18% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.10% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 1.01% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 19 | 0.84% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10082` |
| 21 | 0.67% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.63% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.59% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 24 | 0.59% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15289` |
| 26 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.remove(Entity) line: 456` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14425` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15294` |
| 29 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.moveEntity(Entity) line: 546` |
| 30 | 0.46% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |

GC pauses: 0, total 0 ms

