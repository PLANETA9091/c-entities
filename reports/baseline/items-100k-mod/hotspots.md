# JVM hotspots (JFR ExecutionSample)

Samples: 2740

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 2 | 3.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1203` |
| 3 | 1.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.79% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 5 | 1.79% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 6 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 8 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 9 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.24% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 11 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 1.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 15 | 0.88% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1421` |
| 17 | 0.77% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 18 | 0.73% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 19 | 0.73% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 20 | 0.69% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 21 | 0.66% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 22 | 0.66% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 24 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 25 | 0.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 27 | 0.58% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 28 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 4982` |
| 30 | 0.55% | `java.util.HashMap.getNode(Object) line: 585` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 32 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.51% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 34 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 36 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1197` |
| 38 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.44% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 40 | 0.44% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 41 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 42 | 0.40% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 43 | 0.40% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 4990` |
| 45 | 0.36% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 46 | 0.36% | `java.lang.String.hashCode() line: 2358` |
| 47 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 48 | 0.36% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 49 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 50 | 0.36% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 51 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 52 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 53 | 0.33% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 1699` |
| 55 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 721` |
| 57 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 58 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 59 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 60 | 0.29% | `java.lang.String.equals(Object) line: 1850` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 2 | 3.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1215` |
| 3 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 1.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 6 | 1.79% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1423` |
| 8 | 1.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.42% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 794` |
| 13 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 1.24% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 15 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 16 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 17 | 1.09% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 18 | 1.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 19 | 1.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 20 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 21 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 22 | 0.77% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 24 | 0.69% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 25 | 0.69% | `net.minecraft.world.level.BlockGetter.addCollisionsAlongTravel(LongSet, Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 295` |
| 26 | 0.66% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 28 | 0.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |

GC pauses: 0, total 0 ms

