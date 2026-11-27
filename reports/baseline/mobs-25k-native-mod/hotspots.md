# JVM hotspots (JFR ExecutionSample)

Samples: 3058

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13915` |
| 3 | 2.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 1.83% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10761` |
| 10 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.88% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.82% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 18 | 0.72% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 19 | 0.69% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 21 | 0.52% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.49% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 0.46% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.46% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 28 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 30 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.39% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 32 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 33 | 0.39% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13903` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 2928` |
| 35 | 0.39% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 36 | 0.36% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 38 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.33% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 40 | 0.33% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 41 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 42 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 930` |
| 43 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 45 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.26% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 49 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.23% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1160` |
| 53 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 12552` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2906` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 57 | 0.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 58 | 0.20% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 59 | 0.20% | `java.util.HashMap.getNode(Object) line: 579` |
| 60 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 2 | 4.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 3 | 3.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.71% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13826` |
| 5 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15299` |
| 12 | 0.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10856` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13846` |
| 15 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.82% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 17 | 0.69% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16575` |
| 19 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.52% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.52% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16580` |
| 27 | 0.46% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 28 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.39% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

