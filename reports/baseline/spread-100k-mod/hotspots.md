# JVM hotspots (JFR ExecutionSample)

Samples: 33521

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10202` |
| 2 | 3.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6314` |
| 5 | 1.94% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9600` |
| 7 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 8 | 1.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.01% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.78% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10114` |
| 22 | 0.72% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6821` |
| 30 | 0.61% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 31 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.56% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 34 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 36 | 0.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 37 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 39 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6487` |
| 41 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 46 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 52 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6665` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7431` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 56 | 0.32% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 57 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 58 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9748` |
| 5 | 2.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 9 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6904` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.10% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 15 | 1.02% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.78% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 25 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 28 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7307` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |

GC pauses: 0, total 0 ms

