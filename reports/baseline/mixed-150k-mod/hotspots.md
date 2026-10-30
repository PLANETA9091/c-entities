# JVM hotspots (JFR ExecutionSample)

Samples: 35702

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 3.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 5 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 7 | 2.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 9 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 12 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 17 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 18 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.70% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 32 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 36 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 37 | 0.51% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 39 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.45% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5737` |
| 48 | 0.38% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 49 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.37% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 54 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 55 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 7 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8709` |
| 9 | 1.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 12 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 15 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2170` |
| 22 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 29 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6626` |

GC pauses: 0, total 0 ms

