# JVM hotspots (JFR ExecutionSample)

Samples: 44583

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 7.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 3.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10175` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.96% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 8 | 1.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10811` |
| 12 | 1.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 19 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10723` |
| 23 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 32 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10166` |
| 35 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 38 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 46 | 0.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 47 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.26% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.26% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.24% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 58 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 59 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.22% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 7.33% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.37% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 4.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10358` |
| 5 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 7 | 1.98% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 8 | 1.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 10 | 1.64% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 15 | 1.10% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 19 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.56% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7777` |
| 29 | 0.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |

GC pauses: 0, total 0 ms

