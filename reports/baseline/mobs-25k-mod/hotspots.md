# JVM hotspots (JFR ExecutionSample)

Samples: 20300

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5166` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.77% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2275` |
| 10 | 1.42% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 13 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2256` |
| 19 | 0.80% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.79% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8271` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.66% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.62% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5021` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 31 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 33 | 0.49% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8338` |
| 34 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 35 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6183` |
| 45 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.41% | `java.lang.ref.Reference.get() line: 361` |
| 47 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 48 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 49 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.isNullNibbleVisible() line: 280` |
| 51 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 52 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 53 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 54 | 0.35% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 58 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1748` |
| 3 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2260` |
| 6 | 1.55% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8143` |
| 11 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.21% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 15 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.95% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4971` |
| 21 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.80% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.77% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 26 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms

