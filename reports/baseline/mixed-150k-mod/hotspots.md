# JVM hotspots (JFR ExecutionSample)

Samples: 31795

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 5.87% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 3 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 5 | 3.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10950` |
| 7 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 8 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.78% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 24 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 25 | 0.74% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 32 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 33 | 0.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 34 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 36 | 0.43% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 37 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 38 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10870` |
| 40 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 41 | 0.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 42 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 43 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10305` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 46 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 47 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10862` |
| 50 | 0.32% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10758` |
| 51 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10333` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2404` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 6.67% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 6.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10497` |
| 5 | 3.24% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 6 | 2.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 11 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 14 | 1.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 16 | 1.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 17 | 1.01% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 18 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 22 | 0.82% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 24 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

GC pauses: 0, total 0 ms

