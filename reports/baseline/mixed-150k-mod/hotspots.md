# JVM hotspots (JFR ExecutionSample)

Samples: 34804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 4 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 30 | 0.59% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 32 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8567` |
| 34 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8124` |
| 42 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 45 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.36% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 49 | 0.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 50 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 52 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 55 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.32% | `java.lang.ref.Reference.get() line: 361` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 60 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 6 | 2.34% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.79% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8494` |
| 10 | 1.53% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 12 | 1.47% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6411` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2081` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

