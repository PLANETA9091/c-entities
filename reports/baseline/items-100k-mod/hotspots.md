# JVM hotspots (JFR ExecutionSample)

Samples: 5380

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 2 | 7.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 5.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 4 | 5.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 5 | 3.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 6 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2339` |
| 10 | 1.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 12 | 1.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4600` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3242` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2358` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7923` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 25 | 0.54% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8233` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7925` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1990` |
| 29 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3174` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4455` |
| 33 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3286` |
| 35 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 997` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3086` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4669` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3193` |
| 40 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2334` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 45 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 48 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 49 | 0.19% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 50 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1153` |
| 53 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.15% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 56 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2342` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4454` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3244` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 2 | 9.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 4 | 5.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 7 | 3.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1831` |
| 8 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3084` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3202` |
| 10 | 1.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 11 | 1.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 14 | 1.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3201` |
| 16 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8236` |
| 17 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3182` |
| 21 | 0.54% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2343` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 578` |
| 25 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8228` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 951` |
| 29 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

