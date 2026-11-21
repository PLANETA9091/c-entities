# JVM hotspots (JFR ExecutionSample)

Samples: 2225

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 2 | 4.04% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.24% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.52% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 6 | 2.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10280` |
| 7 | 2.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.75% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 13 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.94% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.90% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 17 | 0.85% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 18 | 0.72% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.67% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15556` |
| 21 | 0.63% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 22 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.54% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 24 | 0.54% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.49% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 26 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.45% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 28 | 0.45% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.45% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 31 | 0.40% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2627` |
| 33 | 0.40% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 34 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 35 | 0.36% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |
| 36 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.31% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 38 | 0.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 39 | 0.31% | `java.util.ArrayList$Itr.next() line: 1058` |
| 40 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13280` |
| 44 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2824` |
| 47 | 0.27% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 48 | 0.22% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 49 | 0.22% | `java.util.ArrayList.elementData(int) line: 412` |
| 50 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 51 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13300` |
| 55 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.18% | `java.io.ByteArrayInputStream.read() line: 147` |
| 57 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 58 | 0.18% | `java.lang.Object.hashCode()` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14677` |
| 60 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 2 | 4.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13230` |
| 4 | 3.78% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13297` |
| 5 | 3.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10319` |
| 10 | 1.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8168` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15541` |
| 17 | 0.85% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13250` |
| 19 | 0.72% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14663` |
| 21 | 0.63% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13258` |
| 23 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.54% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 25 | 0.49% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.49% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.45% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15544` |

GC pauses: 0, total 0 ms

