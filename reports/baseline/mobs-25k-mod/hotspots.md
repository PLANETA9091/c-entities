# JVM hotspots (JFR ExecutionSample)

Samples: 15522

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 7 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 9 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 14 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.84% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.83% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4741` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8034` |
| 26 | 0.68% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 28 | 0.68% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 30 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 31 | 0.60% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 32 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5903` |
| 36 | 0.57% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 39 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 40 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3209` |
| 42 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 43 | 0.47% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.46% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3146` |
| 46 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 47 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 53 | 0.39% | `java.lang.ref.Reference.get() line: 361` |
| 54 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 56 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 57 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 59 | 0.35% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 60 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 3 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 1.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7839` |
| 8 | 1.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 11 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 19 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 20 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.85% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.82% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5899` |
| 27 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 29 | 0.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7822` |
| 30 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

