# JVM hotspots (JFR ExecutionSample)

Samples: 29035

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 6.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 3 | 4.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 5 | 2.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 2.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 7 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.91% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.60% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.59% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3456` |
| 14 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3431` |
| 15 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3220` |
| 16 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3438` |
| 17 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 18 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 1.24% | `java.util.ArrayList.<init>(int) line: 157` |
| 21 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3420` |
| 23 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 1.04% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 1.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 26 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.94% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 28 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3449` |
| 29 | 0.76% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 30 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3426` |
| 31 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3210` |
| 33 | 0.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 34 | 0.62% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 35 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4660` |
| 36 | 0.60% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 37 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.50% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6303` |
| 40 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.46% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 42 | 0.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3432` |
| 44 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3417` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12459` |
| 51 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3425` |
| 55 | 0.30% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 56 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.29% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 58 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.27% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 60 | 0.26% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12451` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 2 | 8.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3453` |
| 3 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.89% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 7 | 1.72% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 1.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.60% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 10 | 1.59% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 11 | 1.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.31% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6468` |
| 15 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 16 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6550` |
| 19 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.75% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 21 | 0.73% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4668` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 24 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8403` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10202` |
| 30 | 0.51% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

