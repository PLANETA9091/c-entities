# JVM hotspots (JFR ExecutionSample)

Samples: 24896

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 5.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 5 | 2.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6320` |
| 6 | 2.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 7 | 2.19% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 8 | 2.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3921` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6166` |
| 16 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4679` |
| 18 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 21 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.76% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 28 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 37 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.57% | `dev.planeta9091.centities.CeHooks.gsMirrorTickRunning(GoalSelector, boolean) line: 4745` |
| 39 | 0.56% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 41 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 45 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.39% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4674` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4694` |
| 50 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 618` |
| 52 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 53 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 56 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.57% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3871` |
| 5 | 2.53% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4519` |
| 7 | 2.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 8 | 2.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 2.17% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 2.02% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 12 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4068` |
| 13 | 1.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.48% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4678` |
| 19 | 1.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 20 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 1.22% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 22 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.98% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 26 | 0.80% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 30 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |

GC pauses: 0, total 0 ms

