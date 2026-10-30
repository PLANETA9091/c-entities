# JVM hotspots (JFR ExecutionSample)

Samples: 4794

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 2 | 9.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 5.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 4 | 4.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 5 | 4.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.40% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 9 | 2.25% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 11 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3380` |
| 14 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5519` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 19 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4790` |
| 21 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8432` |
| 23 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 25 | 0.54% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8122` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 28 | 0.35% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 29 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4868` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1076` |
| 34 | 0.33% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 35 | 0.29% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 36 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 39 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.23% | `java.lang.Integer.valueOf(int) line: 1006` |
| 41 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4739` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3224` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2451` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4789` |
| 47 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 48 | 0.19% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3377` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4835` |
| 51 | 0.19% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 52 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4608` |
| 56 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 57 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 59 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 60 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 4 | 5.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3340` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5519` |
| 6 | 3.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 2.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1910` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3222` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3339` |
| 11 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 12 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 14 | 1.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2435` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 16 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2452` |
| 19 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2081` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8427` |
| 21 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3320` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 25 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.35% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1030` |
| 28 | 0.33% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 29 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.33% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

