# JVM hotspots (JFR ExecutionSample)

Samples: 5663

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.02% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 3.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 1.94% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 12 | 0.90% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 14 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 15 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 18 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 22 | 0.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 30 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 33 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.26% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 37 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 39 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 41 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 42 | 0.23% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 46 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.21% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 49 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4063` |
| 53 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2719` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 57 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 58 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2761` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.85% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 5 | 4.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 6 | 4.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 7 | 3.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 18 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 19 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 21 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.34% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 24 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 25 | 0.32% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 28 | 0.26% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 213` |
| 29 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

