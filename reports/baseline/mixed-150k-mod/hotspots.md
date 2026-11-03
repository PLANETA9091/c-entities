# JVM hotspots (JFR ExecutionSample)

Samples: 31353

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6347` |
| 2 | 3.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10235` |
| 5 | 2.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9633` |
| 7 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5584` |
| 9 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4797` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10147` |
| 21 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 23 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6854` |
| 28 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.55% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9607` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9604` |
| 39 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.setDirty() line: 114` |
| 40 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 47 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 50 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6360` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6520` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 55 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9628` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 59 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9781` |
| 3 | 4.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.83% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 2.78% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 7 | 2.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.22% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 17 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6937` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |
| 24 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6357` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 28 | 0.71% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

