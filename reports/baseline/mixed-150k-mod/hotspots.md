# JVM hotspots (JFR ExecutionSample)

Samples: 34395

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2515` |
| 2 | 6.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.34% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2246` |
| 4 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6583` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10490` |
| 7 | 2.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9888` |
| 12 | 1.38% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2255` |
| 14 | 1.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2244` |
| 17 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 1.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 23 | 0.93% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 24 | 0.86% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.63% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 31 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.59% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 33 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7090` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 37 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7158` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 40 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 41 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9906` |
| 43 | 0.35% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6756` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2483` |
| 47 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 49 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10402` |
| 52 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 53 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4680` |
| 55 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 56 | 0.27% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7700` |
| 57 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2983` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10410` |
| 59 | 0.26% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6934` |
| 60 | 0.26% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2288` |
| 2 | 6.93% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 6.38% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4534` |
| 4 | 6.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10036` |
| 7 | 2.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.96% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.85% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.84% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2327` |
| 13 | 1.40% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2568` |
| 15 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.29% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 17 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 1.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2509` |
| 19 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7173` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.95% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.86% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 25 | 0.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

