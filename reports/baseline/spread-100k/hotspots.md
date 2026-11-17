# JVM hotspots (JFR ExecutionSample)

Samples: 67017

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 0.82% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 17 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 34 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 35 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 38 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 39 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3785` |
| 41 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 44 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 46 | 0.37% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 47 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 56 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.28% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 4 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 2.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.12% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 12 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 15 | 0.78% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 111` |
| 16 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 20 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 23 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 28 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 29 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

