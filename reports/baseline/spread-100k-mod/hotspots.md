# JVM hotspots (JFR ExecutionSample)

Samples: 27552

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3405` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 5 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 6 | 2.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3443` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3407` |
| 12 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3425` |
| 13 | 1.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3422` |
| 14 | 1.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3207` |
| 17 | 1.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 1.24% | `java.util.ArrayList.<init>(int) line: 157` |
| 21 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 1.07% | `java.util.ArrayDeque.elementAt(Object[], int) line: 257` |
| 23 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4887` |
| 25 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3413` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 27 | 0.81% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 28 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3197` |
| 29 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 31 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 33 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.60% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 35 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.58% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3436` |
| 40 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4647` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3404` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3419` |
| 43 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12438` |
| 45 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7377` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6290` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3412` |
| 50 | 0.36% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 51 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.30% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 55 | 0.26% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 56 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.26% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 58 | 0.25% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 59 | 0.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.24% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12446` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 2 | 7.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3440` |
| 3 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 7 | 1.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6455` |
| 9 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.32% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 13 | 1.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6537` |
| 15 | 1.07% | `java.util.ArrayDeque.pollFirst() line: 377` |
| 16 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4655` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 19 | 0.79% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 20 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 24 | 0.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10189` |
| 27 | 0.58% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.43% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8390` |

GC pauses: 0, total 0 ms

