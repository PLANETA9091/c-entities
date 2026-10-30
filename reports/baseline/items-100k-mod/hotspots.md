# JVM hotspots (JFR ExecutionSample)

Samples: 5017

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 2 | 11.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 3 | 5.02% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 6 | 3.13% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 3.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 8 | 2.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3391` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 20 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8443` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.34% | `java.lang.Integer.valueOf(int) line: 1006` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8133` |
| 29 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4807` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2462` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4846` |
| 34 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.26% | `java.util.HashMap.getNode(Object) line: 577` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1087` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3393` |
| 38 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4620` |
| 41 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 42 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8450` |
| 45 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 47 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3323` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4707` |
| 53 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 54 | 0.16% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.14% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.14% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3235` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4619` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.91% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 4 | 5.08% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3351` |
| 6 | 2.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 8 | 2.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3233` |
| 10 | 2.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2446` |
| 11 | 2.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3350` |
| 15 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 17 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8446` |
| 18 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2463` |
| 19 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 21 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.46% | `java.util.HashMap.get(Object) line: 565` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 613` |
| 24 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8438` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3331` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 28 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

