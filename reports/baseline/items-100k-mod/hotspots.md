# JVM hotspots (JFR ExecutionSample)

Samples: 4702

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 2 | 10.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 5.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 5 | 4.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 6 | 3.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 8 | 2.64% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 10 | 2.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 11 | 1.89% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3595` |
| 12 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 1.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5005` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 20 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8337` |
| 24 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8647` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.49% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1121` |
| 29 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5083` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8654` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5050` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5011` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2610` |
| 35 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 38 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 42 | 0.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 43 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4824` |
| 46 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3590` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3527` |
| 49 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.15% | `com.google.common.collect.RegularImmutableMap$KeySet.get(int) line: 326` |
| 52 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5016` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3439` |
| 55 | 0.13% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 56 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5004` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8626` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4823` |
| 59 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 4 | 4.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3555` |
| 5 | 3.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 6 | 3.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1985` |
| 8 | 2.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3437` |
| 10 | 2.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 11 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3554` |
| 12 | 1.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8650` |
| 14 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 15 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 16 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 17 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8642` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2611` |
| 23 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3535` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 26 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1075` |
| 29 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

