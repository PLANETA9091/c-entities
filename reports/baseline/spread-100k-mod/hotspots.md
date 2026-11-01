# JVM hotspots (JFR ExecutionSample)

Samples: 32867

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 4 | 2.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 7 | 2.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 9 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 14 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.99% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 21 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.82% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.75% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 32 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.71% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 37 | 0.62% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 38 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 40 | 0.60% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.54% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4146` |
| 48 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 49 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 50 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 54 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 55 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.38% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6973` |
| 58 | 0.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 59 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4083` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8881` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 7 | 1.85% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 10 | 1.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8933` |
| 13 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 1.36% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 21 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.92% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.88% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 30 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |

GC pauses: 0, total 0 ms

