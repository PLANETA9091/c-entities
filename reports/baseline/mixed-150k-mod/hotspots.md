# JVM hotspots (JFR ExecutionSample)

Samples: 28381

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 2 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 5 | 2.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 6 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 9 | 1.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 25 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.60% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 33 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 35 | 0.51% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 36 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 43 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10169` |
| 47 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 55 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 59 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 4.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.38% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.32% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 6 | 2.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.13% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 20 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.84% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 26 | 0.71% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

