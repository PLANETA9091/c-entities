# JVM hotspots (JFR ExecutionSample)

Samples: 4340

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 2 | 8.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 3 | 5.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2768` |
| 5 | 4.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 6 | 4.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.90% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.88% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2670` |
| 10 | 2.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 2.05% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2721` |
| 12 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 13 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4078` |
| 15 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4783` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 19 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.85% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7682` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7672` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7362` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1962` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7364` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4134` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4117` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1943` |
| 31 | 0.39% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 718` |
| 32 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 33 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7679` |
| 37 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2682` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4020` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 41 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 42 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.18% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2723` |
| 47 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2717` |
| 50 | 0.16% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2575` |
| 52 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 53 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.14% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4071` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4022` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4021` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3933` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 60 | 0.14% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7717` |
| 4 | 6.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 5 | 5.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 6 | 3.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4783` |
| 7 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2573` |
| 9 | 2.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2690` |
| 11 | 2.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 2.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7709` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2670` |
| 14 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7675` |
| 16 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1485` |
| 17 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7667` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 357` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 24 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 26 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |

GC pauses: 0, total 0 ms

