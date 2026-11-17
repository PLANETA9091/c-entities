# JVM hotspots (JFR ExecutionSample)

Samples: 3003

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 2 | 2.63% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 5 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.80% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |
| 13 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.07% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14652` |
| 17 | 0.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.70% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 19 | 0.57% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 22 | 0.50% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 23 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14670` |
| 25 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 30 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 32 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.40% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2471` |
| 36 | 0.40% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14039` |
| 37 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 38 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2477` |
| 39 | 0.37% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12474` |
| 40 | 0.33% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 41 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 42 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 44 | 0.30% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 45 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 47 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 48 | 0.27% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2668` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 50 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 52 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.23% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 54 | 0.23% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 55 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.23% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 57 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 58 | 0.23% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 59 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 60 | 0.20% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 2 | 3.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12424` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12491` |
| 7 | 2.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 8 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.40% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7843` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9594` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13857` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14655` |
| 18 | 0.73% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12444` |
| 20 | 0.70% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 23 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12452` |
| 29 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |

GC pauses: 0, total 0 ms

