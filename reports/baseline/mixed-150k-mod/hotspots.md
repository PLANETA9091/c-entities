# JVM hotspots (JFR ExecutionSample)

Samples: 28738

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6314` |
| 3 | 2.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 5 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9600` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10114` |
| 19 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 21 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 24 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 28 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6821` |
| 33 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 34 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 36 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.43% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 39 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 40 | 0.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 41 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 43 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.41% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 46 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9574` |
| 48 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9571` |
| 51 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 52 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 53 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 10213` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6487` |
| 57 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10122` |
| 59 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9748` |
| 3 | 3.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.94% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.47% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 6 | 2.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.30% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.12% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 15 | 1.01% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7307` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6904` |
| 20 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.77% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6324` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |

GC pauses: 0, total 0 ms

