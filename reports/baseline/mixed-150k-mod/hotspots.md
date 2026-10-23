# JVM hotspots (JFR ExecutionSample)

Samples: 57529

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.85% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1048` |
| 10 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.22% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 13 | 1.18% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 14 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 17 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 24 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.55% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 33 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.54% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 36 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 40 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 44 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 45 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.34% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 47 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 52 | 0.31% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 53 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 54 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 57 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 58 | 0.29% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 60 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1049` |
| 3 | 3.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 2.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 8 | 1.90% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 1.76% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 10 | 1.31% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 11 | 1.19% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 12 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 0.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.74% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 22 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2193` |
| 25 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 27 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.60% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms

