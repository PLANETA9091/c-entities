# JVM hotspots (JFR ExecutionSample)

Samples: 3968

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 8.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 4 | 3.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9906` |
| 5 | 3.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9899` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 9 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5794` |
| 11 | 2.02% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4262` |
| 12 | 1.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 13 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 14 | 1.54% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9859` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 17 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5891` |
| 18 | 1.31% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 1.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9862` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 22 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 24 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5924` |
| 26 | 0.60% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4175` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5877` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10082` |
| 31 | 0.35% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9867` |
| 33 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 34 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 35 | 0.28% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 36 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 40 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5803` |
| 42 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2957` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5598` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5599` |
| 47 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 53 | 0.18% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 54 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5875` |
| 56 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2936` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5817` |
| 59 | 0.15% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5691` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.82% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 3 | 8.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 5 | 5.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4215` |
| 6 | 4.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 7 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 8 | 3.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4079` |
| 9 | 2.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4213` |
| 12 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5877` |
| 15 | 1.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5895` |
| 20 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.40% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4183` |
| 23 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.30% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.28% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 28 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.23% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 30 | 0.23% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |

GC pauses: 0, total 0 ms

