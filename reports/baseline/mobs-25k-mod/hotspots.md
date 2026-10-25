# JVM hotspots (JFR ExecutionSample)

Samples: 14348

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.61% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 3 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 5 | 1.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1404` |
| 6 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.83% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4124` |
| 12 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 1.01% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.91% | `java.util.ArrayDeque.size() line: 647` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.89% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 23 | 0.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.86% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 29 | 0.76% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.72% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 31 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 32 | 0.66% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 33 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 35 | 0.63% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 36 | 0.62% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 37 | 0.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 38 | 0.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 43 | 0.54% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 44 | 0.53% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 45 | 0.52% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3724` |
| 46 | 0.50% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.47% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 48 | 0.45% | `dev.planeta9091.centities.CeHooks.mirrorMaybeTimed(Level, Entity, AABB, Predicate, int) line: 993` |
| 49 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3628` |
| 50 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 52 | 0.45% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 3772` |
| 53 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 57 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 58 | 0.42% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4270` |
| 59 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1057` |
| 3 | 2.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 2.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 601` |
| 7 | 2.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 8 | 2.00% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 9 | 1.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3842` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 1.26% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2494` |
| 18 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 19 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1408` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.90% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.addMovementThisTick(Entity$Movement) line: 1436` |
| 24 | 0.87% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.87% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 28 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 30 | 0.75% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

GC pauses: 0, total 0 ms

