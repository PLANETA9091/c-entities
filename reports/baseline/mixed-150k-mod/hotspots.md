# JVM hotspots (JFR ExecutionSample)

Samples: 46071

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 9.97% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 3.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 5 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11028` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 14 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 16 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 19 | 0.72% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 20 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10940` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 23 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 27 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10383` |
| 34 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.33% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7304` |
| 44 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 45 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 49 | 0.26% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.24% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2897` |
| 54 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 57 | 0.23% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.22% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 60 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 9.97% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 3 | 4.96% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 4.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10575` |
| 5 | 3.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 10 | 1.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 13 | 1.07% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 0.90% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 17 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 19 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 20 | 0.78% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.75% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7803` |
| 25 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 26 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7382` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

