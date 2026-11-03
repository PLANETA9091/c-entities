# JVM hotspots (JFR ExecutionSample)

Samples: 31676

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10235` |
| 2 | 3.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6347` |
| 5 | 2.15% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9633` |
| 7 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.16% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.04% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5584` |
| 18 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.75% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 21 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.73% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 25 | 0.68% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6854` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10147` |
| 28 | 0.60% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 32 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.54% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 38 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6520` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 41 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 44 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 47 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 48 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6698` |
| 50 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7464` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 56 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 58 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9781` |
| 6 | 2.15% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 7 | 1.96% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 12 | 1.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 13 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 0.95% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 18 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6937` |
| 20 | 0.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 27 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6853` |
| 29 | 0.64% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

