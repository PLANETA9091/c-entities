# JVM hotspots (JFR ExecutionSample)

Samples: 41132

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 8.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.00% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 6 | 1.96% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 11 | 1.29% | `java.util.ArrayList.elementData(int) line: 412` |
| 12 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5973` |
| 15 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 18 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10555` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 24 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2636` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 28 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 33 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7243` |
| 41 | 0.34% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 42 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10015` |
| 45 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 46 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 47 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6749` |
| 48 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 49 | 0.29% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 51 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10012` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.25% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6909` |
| 56 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7311` |
| 58 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.24% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 8.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 4.41% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.02% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 6 | 2.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10189` |
| 8 | 2.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 12 | 1.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.29% | `java.util.ArrayList.get(int) line: 429` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.58% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7326` |
| 28 | 0.49% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7729` |
| 30 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

