# JVM hotspots (JFR ExecutionSample)

Samples: 2968

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.07% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 2.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 3 | 2.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 2.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 1.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 8 | 1.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.42% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14528` |
| 11 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.91% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 17 | 0.88% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 18 | 0.77% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 19 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 20 | 0.71% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14546` |
| 22 | 0.61% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.54% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.51% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2429` |
| 31 | 0.44% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.37% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.34% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 39 | 0.34% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 40 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 41 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7689` |
| 44 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.27% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2423` |
| 48 | 0.27% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 52 | 0.24% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 53 | 0.24% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 54 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 55 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 57 | 0.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 58 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 59 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12409` |
| 60 | 0.24% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 2 | 3.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 2.76% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12426` |
| 4 | 2.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12359` |
| 5 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 7 | 1.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9529` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12379` |
| 16 | 0.94% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7780` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14531` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12387` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13792` |
| 21 | 0.71% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.61% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 24 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 26 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.40% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 29 | 0.40% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.moveEntity(Entity) line: 546` |

GC pauses: 0, total 0 ms

