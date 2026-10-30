# JVM hotspots (JFR ExecutionSample)

Samples: 5195

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 2 | 12.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 3 | 5.06% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 6 | 3.50% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 7 | 3.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 8 | 2.89% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5005` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3595` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8647` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 25 | 0.35% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.29% | `java.util.HashMap.getNode(Object) line: 577` |
| 27 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.25% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8337` |
| 32 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.21% | `java.lang.Integer.valueOf(int) line: 1006` |
| 36 | 0.21% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 37 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4824` |
| 40 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4823` |
| 44 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5050` |
| 45 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 47 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 48 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |
| 49 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 51 | 0.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3527` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5011` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4952` |
| 55 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 57 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.13% | `java.util.HashMap.getNode(Object) line: 576` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3590` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 4 | 5.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3555` |
| 6 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 7 | 2.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3437` |
| 8 | 2.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 10 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1985` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 13 | 1.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3554` |
| 15 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2611` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8650` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |
| 20 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.48% | `java.util.HashMap.get(Object) line: 565` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8642` |
| 24 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3535` |
| 26 | 0.31% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 637` |

GC pauses: 0, total 0 ms

