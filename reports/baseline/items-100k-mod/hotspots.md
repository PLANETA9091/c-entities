# JVM hotspots (JFR ExecutionSample)

Samples: 4744

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 2 | 8.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 3 | 6.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 5 | 4.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4006` |
| 6 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.05% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 8 | 3.10% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 10 | 2.36% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 11 | 1.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.69% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3959` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 15 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5419` |
| 17 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 20 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 21 | 1.01% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8767` |
| 26 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9087` |
| 27 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.48% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8769` |
| 30 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5458` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9084` |
| 34 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5491` |
| 36 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5359` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3891` |
| 40 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 41 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 44 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.17% | `java.util.ArrayList.elementData(int) line: 412` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3783` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6150` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3872` |
| 49 | 0.15% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 50 | 0.15% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 51 | 0.15% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3961` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5360` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 55 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.13% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 57 | 0.13% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6147` |
| 60 | 0.11% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.43% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5450` |
| 4 | 6.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 5 | 4.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3912` |
| 6 | 4.05% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 7 | 3.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 8 | 3.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6159` |
| 9 | 3.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 3.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3781` |
| 12 | 2.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3910` |
| 13 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 14 | 1.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3879` |
| 16 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9080` |
| 17 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9072` |
| 22 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 24 | 0.48% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3880` |
| 26 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

