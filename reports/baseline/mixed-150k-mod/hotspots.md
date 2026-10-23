# JVM hotspots (JFR ExecutionSample)

Samples: 47714

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 5.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 3.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 3.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 1.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.23% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 17 | 1.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.97% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.92% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 23 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 25 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.80% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 28 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 30 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 34 | 0.62% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1048` |
| 36 | 0.60% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 39 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.48% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 43 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 48 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 49 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 50 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 52 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 53 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 56 | 0.35% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 57 | 0.34% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 60 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1049` |
| 3 | 3.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 4 | 3.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 3.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 1.96% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 8 | 1.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.44% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 1.30% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 14 | 1.28% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 15 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 19 | 0.96% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 0.88% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 23 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 26 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

