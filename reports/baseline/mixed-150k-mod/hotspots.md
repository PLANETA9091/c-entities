# JVM hotspots (JFR ExecutionSample)

Samples: 36123

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 8.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10303` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10939` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 13 | 1.28% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 18 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 27 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 32 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10340` |
| 36 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10851` |
| 38 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10294` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 41 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 42 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 46 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 47 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 48 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10322` |
| 50 | 0.31% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7135` |
| 51 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 55 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 8.39% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 5.97% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 4.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10486` |
| 5 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.06% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 9 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 12 | 1.31% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 14 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 17 | 1.07% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 19 | 0.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 25 | 0.58% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7777` |
| 29 | 0.47% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 30 | 0.47% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

