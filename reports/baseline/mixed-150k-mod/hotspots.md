# JVM hotspots (JFR ExecutionSample)

Samples: 31794

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 3.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 4 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9543` |
| 6 | 2.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 8 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 10 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9488` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 27 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.48% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 42 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.40% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 46 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8786` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 52 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4366` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 4.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 5 | 2.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9220` |
| 10 | 1.93% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 18 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.59% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

