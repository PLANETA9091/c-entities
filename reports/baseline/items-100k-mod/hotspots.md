# JVM hotspots (JFR ExecutionSample)

Samples: 5247

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4676` |
| 2 | 12.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 3 | 5.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8378` |
| 5 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3377` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3269` |
| 10 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5467` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4740` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3330` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2416` |
| 19 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5469` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8392` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8382` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4777` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.40% | `java.lang.Integer.valueOf(int) line: 1006` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4785` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8072` |
| 30 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.25% | `java.util.HashMap.getNode(Object) line: 577` |
| 34 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3262` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2419` |
| 38 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3174` |
| 39 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 42 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4746` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4559` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8389` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4687` |
| 48 | 0.17% | `java.util.HashMap.getNode(Object) line: 576` |
| 49 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 50 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3275` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8386` |
| 53 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 55 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.13% | `java.lang.Class.cast(Object) line: 3491` |
| 57 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.13% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 59 | 0.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3332` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.36% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8427` |
| 4 | 5.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3290` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5469` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8419` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3172` |
| 9 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2412` |
| 11 | 1.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4777` |
| 14 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1889` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3289` |
| 16 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3269` |
| 17 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8385` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2060` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2420` |
| 21 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8377` |
| 23 | 0.53% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 25 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3270` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 28 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 604` |
| 30 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

