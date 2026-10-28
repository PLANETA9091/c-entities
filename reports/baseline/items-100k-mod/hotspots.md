# JVM hotspots (JFR ExecutionSample)

Samples: 4963

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 2 | 7.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 3 | 4.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2682` |
| 7 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 8 | 3.02% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2584` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3992` |
| 12 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 15 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 0.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7596` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4697` |
| 18 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 21 | 0.44% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 22 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 23 | 0.40% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2635` |
| 25 | 0.38% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7276` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1876` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7278` |
| 29 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 32 | 0.28% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4031` |
| 34 | 0.28% | `java.lang.Double.isFinite(double) line: 1023` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2577` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2637` |
| 38 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1549` |
| 42 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1860` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7586` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3934` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3846` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3847` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3937` |
| 50 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3848` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2482` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7565` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2487` |
| 56 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.16% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 58 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.16% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 60 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 12.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 4 | 4.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7631` |
| 5 | 3.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4697` |
| 6 | 3.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2605` |
| 7 | 3.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 3.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7623` |
| 9 | 3.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2487` |
| 10 | 2.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2604` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2584` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1399` |
| 16 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 20 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2585` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 23 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7589` |
| 24 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.30% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1861` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1865` |
| 29 | 0.28% | `net.minecraft.world.phys.Vec3.isFinite() line: 323` |
| 30 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

