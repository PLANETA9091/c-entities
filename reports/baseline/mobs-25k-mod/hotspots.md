# JVM hotspots (JFR ExecutionSample)

Samples: 14196

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 5 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 12 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 16 | 1.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 19 | 0.99% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 20 | 0.94% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.91% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 24 | 0.87% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 25 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.80% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.79% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 32 | 0.73% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 33 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.67% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.66% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6522` |
| 38 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 39 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.60% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4364` |
| 42 | 0.58% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 44 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2496` |
| 45 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.49% | `java.lang.String.hashCode() line: 2430` |
| 47 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.46% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 49 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4409` |
| 52 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 54 | 0.42% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2669` |
| 55 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 56 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 57 | 0.40% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4522` |
| 58 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 4 | 2.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.14% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 2.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 7 | 2.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 8 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 11 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 18 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 20 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 22 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 23 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.80% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 26 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 27 | 0.74% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4506` |
| 30 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |

GC pauses: 0, total 0 ms

