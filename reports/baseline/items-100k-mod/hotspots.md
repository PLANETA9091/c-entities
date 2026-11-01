# JVM hotspots (JFR ExecutionSample)

Samples: 4421

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 2 | 8.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 3 | 6.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 5 | 4.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 6 | 4.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 8 | 2.69% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3821` |
| 10 | 2.13% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 2.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5281` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 17 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 20 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 22 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8949` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8631` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5320` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8939` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8629` |
| 30 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8946` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 34 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 36 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5353` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 39 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5094` |
| 43 | 0.23% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 44 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.20% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5181` |
| 47 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 49 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5223` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3734` |
| 52 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 54 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 55 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 56 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3816` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2680` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5225` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 4 | 6.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 5 | 5.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3774` |
| 6 | 4.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 7 | 3.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 8 | 2.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 9 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3772` |
| 11 | 2.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 13 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8942` |
| 14 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 15 | 1.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3742` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8934` |
| 21 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 23 | 0.43% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1166` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 26 | 0.36% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.34% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 30 | 0.27% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

