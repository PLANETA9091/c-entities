# JVM hotspots (JFR ExecutionSample)

Samples: 36308

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.23% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.10% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 16 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.66% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.56% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 27 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.44% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 35 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 39 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 41 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 45 | 0.30% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 46 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.29% | `java.util.ArrayDeque.size() line: 647` |
| 48 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.28% | `java.util.Optional.isPresent() line: 154` |
| 52 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.23% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.10% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.93% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.52% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.51% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 26 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms

