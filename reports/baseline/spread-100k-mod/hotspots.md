# JVM hotspots (JFR ExecutionSample)

Samples: 41366

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 15 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 1.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 17 | 1.17% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 18 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 21 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 32 | 0.59% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.56% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 44 | 0.43% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 45 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 47 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.37% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 51 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.36% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 58 | 0.35% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5094` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 3.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.38% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.27% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.17% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 21 | 0.84% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 22 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 25 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

