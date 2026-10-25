# JVM hotspots (JFR ExecutionSample)

Samples: 27419

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4510` |
| 3 | 3.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 7 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 9 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4373` |
| 16 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 22 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 24 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.79% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 28 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.add(Object) line: 367` |
| 31 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 34 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 35 | 0.71% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 36 | 0.69% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 37 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 39 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 40 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.62% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3838` |
| 43 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 48 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 49 | 0.46% | `ca.spottedleaf.concurrentutil.executor.queue.PrioritisedTaskQueue.executeTask() line: 122` |
| 50 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3675` |
| 52 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 55 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 58 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 60 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.75% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.85% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 6 | 2.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 7 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3672` |
| 10 | 1.89% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.54% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 18 | 1.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4022` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4052` |
| 23 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 28 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

