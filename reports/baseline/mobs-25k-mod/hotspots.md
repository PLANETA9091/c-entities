# JVM hotspots (JFR ExecutionSample)

Samples: 19011

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 2.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 6 | 1.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.73% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.09% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 16 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 20 | 0.92% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 21 | 0.90% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 22 | 0.85% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.66% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 30 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.63% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 32 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 34 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 35 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 37 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 41 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 43 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 46 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.43% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 50 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.43% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 53 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 54 | 0.39% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 55 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.28% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 4 | 2.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 1.98% | `net.minecraft.world.entity.Mob.tick() line: 410` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.73% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1362` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 15 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 17 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.02% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 20 | 1.01% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 256` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.90% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 30 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

