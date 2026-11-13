# JVM hotspots (JFR ExecutionSample)

Samples: 2777

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11848` |
| 2 | 3.60% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.41% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.62% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11867` |
| 14 | 0.97% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.83% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 20 | 0.72% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 21 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.54% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 23 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.50% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 26 | 0.50% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 27 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.47% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 29 | 0.43% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 30 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1891` |
| 32 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 34 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 35 | 0.36% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 36 | 0.36% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8942` |
| 37 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 38 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7372` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1871` |
| 42 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.29% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 44 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 45 | 0.29% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 46 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.25% | `java.util.HashMap.getNode(Object) line: 588` |
| 49 | 0.25% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1892` |
| 53 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 55 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 56 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 57 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 58 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.22% | `java.util.Objects.equals(Object, Object) line: 62` |
| 60 | 0.22% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 2 | 3.35% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10255` |
| 3 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 6 | 2.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10188` |
| 7 | 2.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.05% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.62% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10208` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 13 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.22% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 15 | 1.12% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6193` |
| 19 | 0.83% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7372` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10216` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.58% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.54% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.50% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 27 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.moveEntity(Entity) line: 546` |
| 30 | 0.47% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

GC pauses: 0, total 0 ms

