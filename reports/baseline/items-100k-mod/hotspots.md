# JVM hotspots (JFR ExecutionSample)

Samples: 5649

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 6.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 6.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 5 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 4.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.61% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 3.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 1.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 13 | 1.50% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 18 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.48% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4677` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 409` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10801` |
| 25 | 0.34% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.34% | `java.util.ArrayList$Itr.next() line: 1058` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.28% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.28% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 31 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 32 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 33 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.19% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 40 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 41 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6038` |
| 44 | 0.19% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 45 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4672` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 294` |
| 49 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 50 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3317` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3327` |
| 53 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 55 | 0.14% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3330` |
| 57 | 0.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 58 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.12% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 60 | 0.12% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.24% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 5 | 4.89% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.61% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 8 | 3.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4630` |
| 9 | 2.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 10 | 2.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 13 | 1.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11017` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4494` |
| 15 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4628` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 20 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 22 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.30% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 26 | 0.30% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4598` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 29 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

