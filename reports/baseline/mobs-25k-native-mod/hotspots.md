# JVM hotspots (JFR ExecutionSample)

Samples: 3543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11904` |
| 2 | 4.09% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 6 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12128` |
| 12 | 0.68% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 13 | 0.65% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 0.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 0.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.51% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.48% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 21 | 0.48% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 22 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1895` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11923` |
| 26 | 0.40% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 28 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.37% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 30 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 32 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 34 | 0.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 36 | 0.31% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.31% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 38 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 40 | 0.28% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 41 | 0.28% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 42 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 43 | 0.28% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 44 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.25% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 47 | 0.23% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 48 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.23% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 50 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 51 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 7243` |
| 53 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 54 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 55 | 0.23% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 56 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.20% | `java.util.BitSet.expandTo(int) line: 357` |
| 58 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.20% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12128` |
| 2 | 4.18% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10243` |
| 3 | 3.98% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10310` |
| 4 | 3.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5137` |
| 9 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 0.90% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6208` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.76% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7427` |
| 18 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.54% | `java.util.ArrayList.grow(int) line: 238` |
| 21 | 0.54% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 22 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.40% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 26 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 27 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.37% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 30 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

