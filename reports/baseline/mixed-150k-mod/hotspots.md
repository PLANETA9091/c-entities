# JVM hotspots (JFR ExecutionSample)

Samples: 36430

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 2 | 3.48% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 3 | 3.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 5 | 2.82% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 9 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 11 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 21 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8124` |
| 28 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8567` |
| 29 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.49% | `java.lang.String.equals(Object) line: 1926` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 40 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.46% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 42 | 0.46% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 43 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 45 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.38% | `java.lang.ref.Reference.get() line: 361` |
| 50 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 56 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 58 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.31% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 6 | 2.47% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8494` |
| 9 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 11 | 1.55% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 16 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 19 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.68% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2081` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

