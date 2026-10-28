# JVM hotspots (JFR ExecutionSample)

Samples: 6193

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 6.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 5.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 7 | 3.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 2.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 9 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.52% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 12 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 15 | 0.73% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 18 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 22 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 24 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 30 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.23% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 35 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 37 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 39 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 40 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 42 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 43 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 44 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2719` |
| 48 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 50 | 0.16% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 51 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 52 | 0.15% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 53 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 55 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.15% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4120` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 4.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 6 | 4.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 7 | 4.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 10 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 12 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 18 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 19 | 0.48% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 20 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 22 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 24 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.27% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 28 | 0.24% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 29 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

