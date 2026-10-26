# JVM hotspots (JFR ExecutionSample)

Samples: 36371

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.92% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 0.88% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.73% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.63% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 25 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.45% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 27 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.42% | `java.lang.String.hashCode() line: 2430` |
| 31 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.36% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 35 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 36 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.35% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 40 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 41 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 43 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 44 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 49 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 50 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.25% | `java.util.ArrayDeque.size() line: 647` |
| 55 | 0.25% | `java.util.Optional.isPresent() line: 154` |
| 56 | 0.25% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 60 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 0.92% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.76% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.60% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 21 | 0.48% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 25 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 27 | 0.45% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 29 | 0.44% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.44% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

