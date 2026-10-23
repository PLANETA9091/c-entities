# JVM hotspots (JFR ExecutionSample)

Samples: 14306

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.22% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 4 | 1.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 1.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 13 | 1.09% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 18 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 20 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 26 | 0.67% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.66% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 32 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.56% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 40 | 0.46% | `java.util.Optional.get() line: 142` |
| 41 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 44 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 55 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.34% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 58 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.31% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.22% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 3 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 1.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 7 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 1.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 17 | 1.08% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1255` |
| 19 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 923` |
| 21 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 23 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |

GC pauses: 0, total 0 ms

