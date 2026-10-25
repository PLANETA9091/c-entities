# JVM hotspots (JFR ExecutionSample)

Samples: 3973

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |
| 3 | 8.46% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 4 | 7.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 5 | 4.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 6 | 2.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2058` |
| 7 | 2.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2146` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.79% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2020` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2084` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 14 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3341` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 17 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 18 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2809` |
| 19 | 0.98% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2193` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 25 | 0.50% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1423` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3347` |
| 29 | 0.38% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 30 | 0.35% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2095` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1404` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2142` |
| 34 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 37 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.23% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 39 | 0.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3462` |
| 41 | 0.23% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2012` |
| 43 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1152` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 46 | 0.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 47 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.18% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2005` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2062` |
| 51 | 0.18% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2148` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2143` |
| 54 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 55 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2854` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2815` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2846` |
| 59 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 883` |
| 60 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2494` |
| 2 | 10.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 8.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 8.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 7.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3462` |
| 6 | 5.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2846` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2045` |
| 9 | 2.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2009` |
| 12 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4211` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1057` |
| 16 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2020` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2116` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 23 | 0.48% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.40% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks$RefField.getLong(Object) line: 2478` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 697` |
| 28 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1408` |
| 30 | 0.25% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |

GC pauses: 0, total 0 ms

