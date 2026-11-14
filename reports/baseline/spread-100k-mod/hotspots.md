# JVM hotspots (JFR ExecutionSample)

Samples: 28452

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 6.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3654` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3598` |
| 4 | 3.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 5 | 3.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 6 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3628` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3627` |
| 10 | 1.61% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3618` |
| 14 | 1.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.47% | `java.util.ArrayList.<init>(int) line: 157` |
| 17 | 1.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3615` |
| 18 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3612` |
| 19 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3678` |
| 21 | 1.25% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 1.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3400` |
| 23 | 1.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3600` |
| 24 | 1.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 25 | 1.03% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.96% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.91% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 28 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3629` |
| 30 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.83% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 33 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3390` |
| 36 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3597` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4891` |
| 39 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 41 | 0.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 42 | 0.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 43 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 44 | 0.41% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 51 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7790` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3605` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12860` |
| 55 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 56 | 0.26% | `java.util.EnumMap.get(Object) line: 248` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 58 | 0.25% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 59 | 0.25% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 60 | 0.24% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 2 | 8.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3633` |
| 3 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.60% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 8 | 1.60% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 9 | 1.52% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 10 | 1.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3546` |
| 12 | 1.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 15 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 16 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 0.93% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.93% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8803` |
| 25 | 0.49% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10602` |
| 28 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |

GC pauses: 0, total 0 ms

