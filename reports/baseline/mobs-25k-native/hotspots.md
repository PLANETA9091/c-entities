# JVM hotspots (JFR ExecutionSample)

Samples: 41972

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.96% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 12 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.55% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 21 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.48% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 27 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 39 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 48 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 49 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 50 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.lambda$forEachModifier$2(EquipmentSlot, BiConsumer, Holder, int) line: 405` |
| 51 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.26% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.26% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 57 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.23% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 59 | 0.22% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.22% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 1.14% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.97% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 9 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.85% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 11 | 0.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 12 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 18 | 0.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 20 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 21 | 0.55% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 22 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.52% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.48% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

