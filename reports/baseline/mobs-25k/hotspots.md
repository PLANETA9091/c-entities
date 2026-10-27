# JVM hotspots (JFR ExecutionSample)

Samples: 27857

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 5.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.41% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.07% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 1.03% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.82% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.82% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.70% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 27 | 0.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 29 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.62% | `java.lang.String.hashCode() line: 2430` |
| 31 | 0.61% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.53% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 39 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.49% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 41 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.47% | `java.util.ArrayDeque.size() line: 647` |
| 45 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 46 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 49 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 54 | 0.34% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 56 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 59 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.41% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.25% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 12 | 1.07% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 17 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.84% | `java.util.HashMap.containsKey(Object) line: 603` |
| 19 | 0.82% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 21 | 0.79% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.69% | `net.minecraft.world.entity.Mob.baseTick() line: 353` |
| 27 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 29 | 0.66% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.66% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms

