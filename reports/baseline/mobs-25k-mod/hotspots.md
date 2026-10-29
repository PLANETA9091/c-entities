# JVM hotspots (JFR ExecutionSample)

Samples: 16253

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 5 | 1.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 9 | 1.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 11 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2142` |
| 14 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.73% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.70% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8192` |
| 22 | 0.66% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.57% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 25 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 34 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.44% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2123` |
| 38 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 42 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 43 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3343` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.34% | `java.lang.ThreadLocal.get() line: 171` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 59 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.33% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1665` |
| 3 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7997` |
| 7 | 1.46% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7972` |
| 11 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2127` |
| 12 | 1.05% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.91% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 17 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4825` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 23 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 26 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 28 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 30 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

