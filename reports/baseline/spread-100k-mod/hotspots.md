# JVM hotspots (JFR ExecutionSample)

Samples: 33475

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10134` |
| 2 | 3.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6262` |
| 6 | 1.81% | `java.lang.ref.Reference.get() line: 361` |
| 7 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9532` |
| 8 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5494` |
| 9 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.31% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.28% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.10% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.79% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 21 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 23 | 0.78% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.74% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 25 | 0.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10046` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 37 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2765` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6442` |
| 42 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 45 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 48 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6764` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4056` |
| 52 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 55 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6275` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4402` |
| 58 | 0.29% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7374` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9680` |
| 6 | 2.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9679` |
| 8 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2106` |
| 13 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.28% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 15 | 1.26% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 191` |
| 16 | 1.08% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 17 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6847` |
| 19 | 1.05% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6453` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

