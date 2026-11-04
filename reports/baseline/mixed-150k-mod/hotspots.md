# JVM hotspots (JFR ExecutionSample)

Samples: 34056

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 7.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.48% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 6 | 2.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 2.01% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10678` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 10 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10076` |
| 13 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 15 | 1.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2636` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10590` |
| 29 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 32 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 40 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 41 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 43 | 0.31% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 46 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 47 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.29% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |
| 51 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 52 | 0.28% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10486` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.27% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 56 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 59 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.24% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 7.62% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 5.97% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 4.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |
| 6 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.28% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.04% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 10 | 1.88% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 14 | 1.10% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.09% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 21 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.58% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7734` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 29 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.49% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

