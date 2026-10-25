# JVM hotspots (JFR ExecutionSample)

Samples: 44473

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.24% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4065` |
| 3 | 3.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3148` |
| 4 | 3.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 3.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1493` |
| 6 | 2.93% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 7 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 12 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 15 | 1.18% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 16 | 1.07% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 3948` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 19 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 22 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1422` |
| 24 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3138` |
| 27 | 0.78% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 28 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.72% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 35 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 36 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 38 | 0.54% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 39 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 40 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.48% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 42 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2750` |
| 44 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3486` |
| 46 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2107` |
| 47 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 49 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 615` |
| 51 | 0.36% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 56 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.66% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 4.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.99% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 600` |
| 5 | 2.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.82% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2751` |
| 7 | 2.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1056` |
| 8 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2355` |
| 11 | 1.52% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.48% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3666` |
| 16 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 1.19% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 20 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 0.97% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 25 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 27 | 0.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 28 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

