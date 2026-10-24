# JVM hotspots (JFR ExecutionSample)

Samples: 39224

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 5 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.04% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.91% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 17 | 0.73% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.68% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.49% | `java.lang.String.hashCode() line: 2430` |
| 26 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 29 | 0.45% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 30 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 32 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 38 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 41 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.32% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.27% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 54 | 0.25% | `java.util.Optional.isPresent() line: 154` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.25% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 7.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.04% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.87% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 15 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 27 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 28 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.45% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 30 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

