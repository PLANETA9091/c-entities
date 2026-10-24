# JVM hotspots (JFR ExecutionSample)

Samples: 40435

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.38% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 2.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 14 | 1.35% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.30% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 16 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1361` |
| 24 | 0.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 25 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 33 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.57% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 37 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 42 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 45 | 0.42% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1999` |
| 51 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 4.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 3.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 3.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 8 | 1.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 9 | 1.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 10 | 1.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1362` |
| 13 | 1.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.42% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.32% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1371` |
| 18 | 1.22% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2464` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 24 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 27 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2531` |

GC pauses: 0, total 0 ms

