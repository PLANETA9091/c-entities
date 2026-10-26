# JVM hotspots (JFR ExecutionSample)

Samples: 4208

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3613` |
| 3 | 5.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 4.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 5 | 4.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.90% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3119` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 11 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.67% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.52% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.50% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 19 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 20 | 0.38% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 21 | 0.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 23 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 24 | 0.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 28 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 32 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3156` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3125` |
| 35 | 0.26% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3644` |
| 37 | 0.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3164` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 42 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 43 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3793` |
| 45 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3639` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 49 | 0.17% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1154` |
| 51 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2054` |
| 54 | 0.14% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 5267` |
| 56 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 57 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 58 | 0.12% | `java.util.HashMap.put(Object, Object) line: 619` |
| 59 | 0.12% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 60 | 0.12% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 2 | 8.70% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 4 | 5.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3793` |
| 5 | 4.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 7 | 3.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 9 | 2.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3156` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5414` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 13 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 16 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.62% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 18 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 19 | 0.50% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2800` |
| 20 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 22 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 27 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2091` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 782` |

GC pauses: 0, total 0 ms

