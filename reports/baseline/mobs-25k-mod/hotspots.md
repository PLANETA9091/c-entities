# JVM hotspots (JFR ExecutionSample)

Samples: 13916

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 5.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.40% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 7 | 1.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 10 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 11 | 1.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |
| 17 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10678` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 21 | 0.83% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7858` |
| 24 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.77% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4769` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6914` |
| 29 | 0.68% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 30 | 0.67% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7092` |
| 31 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10076` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10590` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.55% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.51% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 41 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 44 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 47 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10771` |
| 50 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1580` |
| 53 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.35% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 55 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 57 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 58 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 5.96% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 5.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 4 | 3.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.92% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 2.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7331` |
| 7 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |
| 11 | 1.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.03% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7247` |
| 21 | 0.82% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 22 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7404` |
| 25 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7854` |
| 27 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.72% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.70% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |

GC pauses: 0, total 0 ms

