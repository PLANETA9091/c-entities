# JVM hotspots (JFR ExecutionSample)

Samples: 32920

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 2.45% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 4 | 2.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 5 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 7 | 1.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.04% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.84% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 28 | 0.67% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.64% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 33 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 35 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.50% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 40 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 41 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 43 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 48 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.31% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 58 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 2 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 7 | 2.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 12 | 1.29% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.18% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.07% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 19 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.86% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.84% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |

GC pauses: 0, total 0 ms

