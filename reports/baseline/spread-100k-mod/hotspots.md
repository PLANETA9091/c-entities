# JVM hotspots (JFR ExecutionSample)

Samples: 25473

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3698` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3642` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7056` |
| 6 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3656` |
| 8 | 2.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3659` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3671` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 12988` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3722` |
| 14 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3672` |
| 15 | 1.43% | `java.util.ArrayList.<init>(int) line: 157` |
| 16 | 1.29% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 17 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3444` |
| 20 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3746` |
| 21 | 1.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3662` |
| 22 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 1.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.98% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.91% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 28 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 30 | 0.88% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.81% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 32 | 0.78% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 33 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3644` |
| 34 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.66% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3641` |
| 37 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 38 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4935` |
| 39 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3649` |
| 40 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 43 | 0.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 44 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.40% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 47 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.39% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12900` |
| 51 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.34% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 53 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3643` |
| 55 | 0.28% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.27% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 57 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.26% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 59 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2921` |
| 2 | 6.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3677` |
| 3 | 3.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3590` |
| 10 | 1.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.29% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 12 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4943` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7818` |
| 18 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.84% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 20 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.77% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8851` |
| 25 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10650` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7838` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.getSharedFlag$(Entity, int) line: 9250` |
| 30 | 0.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms

