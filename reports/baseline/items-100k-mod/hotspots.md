# JVM hotspots (JFR ExecutionSample)

Samples: 5626

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 2 | 10.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4214` |
| 3 | 5.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 5 | 4.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.09% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2909` |
| 8 | 2.60% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2801` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4220` |
| 14 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4251` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4934` |
| 18 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2862` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2054` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7833` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7513` |
| 24 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7823` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7515` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1727` |
| 31 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4283` |
| 33 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4259` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4075` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2035` |
| 38 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2794` |
| 44 | 0.23% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2864` |
| 46 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 47 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4162` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2807` |
| 50 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2700` |
| 52 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4931` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 55 | 0.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2813` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4165` |
| 59 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.91% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7868` |
| 4 | 5.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4251` |
| 5 | 4.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7860` |
| 6 | 4.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4934` |
| 8 | 2.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2822` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2704` |
| 10 | 2.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1577` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2821` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2801` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2039` |
| 20 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.43% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 22 | 0.41% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2802` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7826` |
| 25 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 27 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.util.ParallelSearchRadiusIteration.generateBFSOrder(int) line: 344` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

