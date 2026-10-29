# JVM hotspots (JFR ExecutionSample)

Samples: 14826

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.83% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.83% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 19 | 0.81% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 26 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 28 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 29 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6109` |
| 31 | 0.59% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 32 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 34 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3415` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8264` |
| 36 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 37 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 38 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8142` |
| 41 | 0.49% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 42 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3352` |
| 45 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 46 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.42% | `java.lang.ref.Reference.get() line: 361` |
| 51 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 56 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 1.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8069` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 18 | 0.84% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4897` |
| 21 | 0.80% | `net.minecraft.world.item.ItemStack.isBroken() line: 585` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.76% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6105` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |

GC pauses: 0, total 0 ms

