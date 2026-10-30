# JVM hotspots (JFR ExecutionSample)

Samples: 5899

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 2 | 10.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 3 | 5.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 7 | 2.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 10 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.03% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4600` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2358` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8243` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2339` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1990` |
| 23 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3242` |
| 25 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4455` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7925` |
| 28 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3286` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4639` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4543` |
| 34 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4669` |
| 37 | 0.24% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 38 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8233` |
| 40 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 42 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3079` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7923` |
| 45 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8237` |
| 49 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 50 | 0.19% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 51 | 0.19% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2333` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3187` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4593` |
| 55 | 0.19% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 56 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4454` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4456` |
| 60 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 5 | 4.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 6 | 4.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 7 | 3.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3202` |
| 8 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 10 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3084` |
| 11 | 2.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1831` |
| 12 | 1.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2343` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3201` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 578` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.47% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8236` |
| 23 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.util.ParallelSearchRadiusIteration.generateBFSOrder(int) line: 344` |
| 25 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8228` |
| 27 | 0.27% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 28 | 0.27% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 546` |
| 29 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |

GC pauses: 0, total 0 ms

