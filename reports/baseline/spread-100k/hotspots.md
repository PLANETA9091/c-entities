# JVM hotspots (JFR ExecutionSample)

Samples: 69294

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.02% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 15 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 20 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 28 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.52% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 32 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.46% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 38 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 39 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.43% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 43 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 46 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3785` |
| 48 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 53 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 4 | 3.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 9 | 1.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.74% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 11 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.02% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 18 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 25 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.52% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

