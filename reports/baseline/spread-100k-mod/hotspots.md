# JVM hotspots (JFR ExecutionSample)

Samples: 23737

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 3 | 4.78% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3404` |
| 5 | 2.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 6 | 2.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 7 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 2.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3417` |
| 11 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3421` |
| 13 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3406` |
| 14 | 1.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 16 | 1.44% | `java.util.ArrayList.<init>(int) line: 157` |
| 17 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3442` |
| 18 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3424` |
| 19 | 1.06% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 20 | 1.04% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3206` |
| 22 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 1.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.95% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 26 | 0.88% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 27 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3412` |
| 28 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 30 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 32 | 0.75% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 33 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3196` |
| 34 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4646` |
| 35 | 0.68% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3403` |
| 37 | 0.56% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.54% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 39 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.51% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 43 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3463` |
| 44 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12437` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6289` |
| 46 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 48 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 50 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 53 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 54 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3411` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 12333` |
| 57 | 0.31% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 2 | 6.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3439` |
| 3 | 4.78% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 4 | 4.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.62% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3352` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 13 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 16 | 0.90% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 17 | 0.83% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 111` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 19 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 22 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.56% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10188` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6536` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8389` |

GC pauses: 0, total 0 ms

