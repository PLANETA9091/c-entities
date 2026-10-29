# JVM hotspots (JFR ExecutionSample)

Samples: 15182

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 6 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.66% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 9 | 1.53% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.44% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 13 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2054` |
| 15 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 1.06% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8013` |
| 21 | 1.04% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 1.01% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 24 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 26 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.82% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5949` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2035` |
| 30 | 0.74% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 33 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7958` |
| 34 | 0.65% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 35 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4787` |
| 38 | 0.61% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8080` |
| 39 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 41 | 0.59% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.59% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3255` |
| 44 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3192` |
| 46 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 47 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 50 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 52 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.40% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 60 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1577` |
| 3 | 2.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.00% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7885` |
| 7 | 1.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 9 | 1.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 11 | 1.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2039` |
| 13 | 1.44% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 14 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 18 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.02% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 20 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 22 | 0.94% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7868` |
| 26 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.82% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 29 | 0.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4737` |
| 30 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms

