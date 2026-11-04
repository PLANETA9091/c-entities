# JVM hotspots (JFR ExecutionSample)

Samples: 35646

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 9.00% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.45% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10677` |
| 7 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10075` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.07% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2636` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.68% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 25 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 31 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 32 | 0.50% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 33 | 0.50% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 34 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 36 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 38 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6754` |
| 39 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 41 | 0.38% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10485` |
| 42 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 43 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 45 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10093` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10589` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 53 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10597` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 57 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4833` |
| 59 | 0.27% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7858` |
| 60 | 0.27% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 9.00% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 6.77% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 5.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 7 | 2.17% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.44% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 13 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 18 | 0.95% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.86% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.78% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 25 | 0.72% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 26 | 0.65% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

GC pauses: 0, total 0 ms

