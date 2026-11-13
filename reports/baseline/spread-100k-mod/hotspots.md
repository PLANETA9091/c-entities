# JVM hotspots (JFR ExecutionSample)

Samples: 24043

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.10% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 3 | 4.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 5 | 2.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6664` |
| 7 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.71% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3438` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3431` |
| 12 | 1.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3426` |
| 16 | 1.14% | `java.util.ArrayList.<init>(int) line: 157` |
| 17 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.10% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 1.04% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 22 | 0.99% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 23 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3220` |
| 24 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3420` |
| 25 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 27 | 0.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 28 | 0.89% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.83% | `java.util.ArrayDeque.elementAt(Object[], int) line: 257` |
| 30 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3456` |
| 31 | 0.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 32 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4660` |
| 34 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3210` |
| 35 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.55% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 37 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.53% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.48% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 41 | 0.47% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 42 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 44 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12459` |
| 47 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7390` |
| 49 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3449` |
| 51 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3425` |
| 54 | 0.31% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 56 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.replaceFrom(AttributeInstance) line: 176` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 58 | 0.30% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12451` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 2 | 5.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3453` |
| 3 | 5.10% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 4 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 9 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 11 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4668` |
| 16 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 17 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.83% | `java.util.ArrayDeque.pollFirst() line: 377` |
| 19 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 21 | 0.76% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10202` |
| 26 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8403` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10269` |

GC pauses: 0, total 0 ms

