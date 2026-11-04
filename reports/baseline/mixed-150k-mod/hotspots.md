# JVM hotspots (JFR ExecutionSample)

Samples: 50623

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 12.92% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 4 | 2.78% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 5 | 2.04% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10916` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 13 | 1.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 22 | 0.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 23 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10828` |
| 31 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 32 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.36% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 39 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 40 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 42 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 46 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 47 | 0.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.24% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 52 | 0.24% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2412` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |
| 58 | 0.20% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10299` |
| 60 | 0.20% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 12.92% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.06% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 5 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.81% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 7 | 2.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.87% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 16 | 1.02% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 18 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 20 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 24 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.50% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 26 | 0.49% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 30 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

