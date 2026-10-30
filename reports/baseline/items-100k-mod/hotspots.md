# JVM hotspots (JFR ExecutionSample)

Samples: 5920

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 2 | 12.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 3 | 4.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.22% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3534` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 8 | 2.74% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3426` |
| 10 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2520` |
| 11 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 13 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4897` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4934` |
| 18 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5626` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8549` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3487` |
| 23 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8539` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.35% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4715` |
| 32 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2510` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4942` |
| 34 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4716` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 36 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 37 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 38 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8229` |
| 40 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.22% | `java.lang.Integer.valueOf(int) line: 1006` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3324` |
| 44 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2511` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3419` |
| 47 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 49 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3438` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8543` |
| 52 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3331` |
| 55 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 56 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 59 | 0.15% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.15% | `java.util.HashMap.getNode(Object) line: 577` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.53% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 4.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8584` |
| 4 | 4.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8576` |
| 6 | 3.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3447` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5626` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3329` |
| 9 | 2.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2516` |
| 10 | 1.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1970` |
| 11 | 1.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4934` |
| 14 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3446` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3426` |
| 17 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2155` |
| 19 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8542` |
| 21 | 0.46% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2533` |
| 23 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 622` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3427` |
| 28 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.30% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

