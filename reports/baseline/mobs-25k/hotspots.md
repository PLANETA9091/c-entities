# JVM hotspots (JFR ExecutionSample)

Samples: 36975

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.98% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.84% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.84% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.59% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.51% | `net.minecraft.world.entity.Entity.push(Entity) line: 2472` |
| 24 | 0.51% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 26 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 27 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 35 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 39 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.28% | `java.lang.Double.isFinite(double) line: 1023` |
| 48 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 49 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 50 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 51 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.25% | `java.util.Optional.isPresent() line: 154` |
| 57 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.24% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 59 | 0.24% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 60 | 0.24% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 0.98% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.88% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 9 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.62% | `net.minecraft.world.entity.LivingEntity.push(Entity) line: 2948` |
| 14 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.53% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.52% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 22 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.49% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 26 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 27 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.45% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

