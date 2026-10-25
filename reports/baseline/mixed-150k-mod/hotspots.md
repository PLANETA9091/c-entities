# JVM hotspots (JFR ExecutionSample)

Samples: 37925

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4510` |
| 4 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 7 | 2.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 9 | 1.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 13 | 1.38% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.32% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4373` |
| 16 | 1.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 18 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.98% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 21 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.90% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 26 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 29 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 33 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 35 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 37 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.45% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 41 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3838` |
| 48 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.35% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 59 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.97% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 4 | 3.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 7 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 11 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.39% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 13 | 1.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3672` |
| 17 | 1.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 19 | 1.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.91% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.90% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.76% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 27 | 0.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4022` |

GC pauses: 0, total 0 ms

