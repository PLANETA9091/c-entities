# JVM hotspots (JFR ExecutionSample)

Samples: 4872

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 2 | 8.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 3 | 5.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 4 | 4.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 5 | 3.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 11 | 1.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 14 | 1.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 16 | 1.44% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3391` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 19 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8443` |
| 25 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8133` |
| 27 | 0.49% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.37% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4879` |
| 30 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 32 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1087` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 35 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4846` |
| 38 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2462` |
| 40 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 43 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3323` |
| 45 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8447` |
| 47 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8450` |
| 48 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4748` |
| 53 | 0.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 54 | 0.12% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3388` |
| 55 | 0.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3235` |
| 56 | 0.12% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 57 | 0.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3435` |
| 58 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4620` |
| 59 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4807` |
| 60 | 0.12% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.02% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 4 | 4.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3351` |
| 5 | 3.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3233` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2446` |
| 11 | 1.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 13 | 1.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3350` |
| 14 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2463` |
| 15 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8446` |
| 16 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 17 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 18 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 20 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8438` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3331` |
| 26 | 0.43% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.27% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

