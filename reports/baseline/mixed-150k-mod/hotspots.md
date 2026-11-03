# JVM hotspots (JFR ExecutionSample)

Samples: 25733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 3 | 4.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 4 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 10 | 1.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.75% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 1.15% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 19 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.95% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 24 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 26 | 0.70% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 31 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.61% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 33 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 37 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.51% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.50% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 43 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10281` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 50 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10169` |
| 52 | 0.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 53 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 59 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.31% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.14% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 15 | 1.27% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 16 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 17 | 1.15% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 18 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.11% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 24 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 25 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.tick() line: 3481` |

GC pauses: 0, total 0 ms

