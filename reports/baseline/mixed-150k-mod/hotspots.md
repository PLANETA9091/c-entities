# JVM hotspots (JFR ExecutionSample)

Samples: 35826

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 2.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 16 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 18 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 21 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 22 | 1.00% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.87% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.67% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1361` |
| 35 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.53% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 37 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 39 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.44% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 41 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 43 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1982` |
| 45 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.37% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.33% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 58 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 2.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 1.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 7 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 9 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.24% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1371` |
| 18 | 1.07% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.00% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 22 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1362` |
| 23 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2447` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 25 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.78% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

