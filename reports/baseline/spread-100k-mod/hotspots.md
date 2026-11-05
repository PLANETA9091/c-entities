# JVM hotspots (JFR ExecutionSample)

Samples: 40895

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 2 | 6.15% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 5 | 2.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10303` |
| 7 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10939` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 10 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 15 | 1.01% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.81% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.68% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 32 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 36 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.52% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 41 | 0.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 42 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 45 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.44% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 49 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3179` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4835` |
| 54 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 56 | 0.36% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 57 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6797` |
| 60 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 2 | 6.15% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10486` |
| 5 | 3.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 6 | 2.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.47% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 12 | 1.44% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 15 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 19 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 20 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 24 | 0.68% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 25 | 0.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 30 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

GC pauses: 0, total 0 ms

