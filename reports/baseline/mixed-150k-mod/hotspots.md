# JVM hotspots (JFR ExecutionSample)

Samples: 28203

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 3.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.13% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 5 | 2.28% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 6 | 1.77% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 8 | 1.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 11 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 18 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 21 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 26 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 29 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.70% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 31 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 35 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10092` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10588` |
| 39 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.47% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 41 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 48 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7092` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 53 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2402` |
| 55 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 57 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10596` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 4.63% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 4.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.13% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10222` |
| 6 | 2.36% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 7 | 2.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.14% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 11 | 1.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.71% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 16 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 23 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 25 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7734` |
| 27 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 28 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

