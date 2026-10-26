# JVM hotspots (JFR ExecutionSample)

Samples: 30706

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 3.20% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 5 | 2.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 8 | 1.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 16 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 19 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.90% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 22 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 29 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.57% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 34 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 37 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.54% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 41 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 45 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 46 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 48 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 49 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 51 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 54 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 55 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 57 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2496` |
| 60 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.21% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.46% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 5 | 2.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.30% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 7 | 2.27% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 2.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 11 | 1.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.61% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 13 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.56% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 1.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 18 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 19 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 20 | 1.20% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 1.17% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.89% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 30 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |

GC pauses: 0, total 0 ms

