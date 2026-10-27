# JVM hotspots (JFR ExecutionSample)

Samples: 37849

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.11% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 1.04% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.74% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 17 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.63% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.58% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 26 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 29 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 31 | 0.49% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.48% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 34 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 36 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 39 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.37% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.28% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.27% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.26% | `java.util.ArrayDeque.size() line: 647` |
| 54 | 0.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 58 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.25% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.11% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.96% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 17 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.61% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 19 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 21 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.55% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 26 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.49% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 30 | 0.49% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

