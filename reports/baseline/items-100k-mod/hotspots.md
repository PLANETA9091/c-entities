# JVM hotspots (JFR ExecutionSample)

Samples: 4026

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 6.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 4 | 4.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 5 | 4.00% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2273` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2164` |
| 19 | 0.60% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.47% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7167` |
| 24 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 26 | 0.42% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 28 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3573` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2175` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 31 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 33 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2073` |
| 35 | 0.32% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7171` |
| 37 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 38 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 40 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 41 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 43 | 0.20% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 44 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 45 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 47 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 50 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.17% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2160` |
| 54 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.15% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 56 | 0.15% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 57 | 0.15% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 59 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5180` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2222` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 6.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 3 | 6.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 4 | 5.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 5 | 4.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 7 | 3.30% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 11 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 13 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2196` |
| 16 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7170` |
| 17 | 0.60% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 18 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2090` |
| 23 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.42% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.appendList(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 983` |
| 25 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7162` |
| 28 | 0.25% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |

GC pauses: 0, total 0 ms

