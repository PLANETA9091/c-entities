# JVM hotspots (JFR ExecutionSample)

Samples: 3276

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 2 | 6.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 3 | 5.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 4 | 4.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.27% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3527` |
| 13 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7127` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2252` |
| 20 | 0.61% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 22 | 0.52% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2154` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2143` |
| 27 | 0.37% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 28 | 0.37% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2100` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3564` |
| 32 | 0.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.31% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3572` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1448` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4234` |
| 38 | 0.24% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 39 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 42 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 43 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 45 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 47 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 54 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isMergeEmpty(CachedShapeData, CachedShapeData, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, int) line: 1136` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.findFloor(double[], double, double, int, int) line: 180` |
| 56 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 57 | 0.15% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 58 | 0.15% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1322` |
| 60 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 8.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 3 | 6.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 4 | 6.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2057` |
| 6 | 3.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3564` |
| 8 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 9 | 2.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 12 | 1.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7130` |
| 15 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2069` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7122` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2175` |
| 19 | 0.61% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 20 | 0.58% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 23 | 0.43% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2154` |
| 25 | 0.40% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 27 | 0.37% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2137` |

GC pauses: 0, total 0 ms

