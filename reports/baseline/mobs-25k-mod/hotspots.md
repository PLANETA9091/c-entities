# JVM hotspots (JFR ExecutionSample)

Samples: 12456

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2657` |
| 2 | 3.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 3 | 2.51% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2388` |
| 4 | 2.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.62% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 7 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10632` |
| 8 | 1.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.49% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7300` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7232` |
| 12 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2386` |
| 14 | 1.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1280` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.96% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7076` |
| 20 | 0.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.83% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4821` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.78% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6898` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10030` |
| 27 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 30 | 0.68% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 31 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4758` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7842` |
| 35 | 0.55% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10178` |
| 38 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 42 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 44 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6725` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.40% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6925` |
| 48 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 52 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10544` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 58 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 60 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2430` |
| 2 | 3.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 3.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 4 | 2.91% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 5 | 2.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7315` |
| 6 | 2.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.53% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 10 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7300` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2710` |
| 14 | 1.24% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7388` |
| 17 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 1.08% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7231` |
| 22 | 1.00% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10178` |
| 24 | 0.96% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 28 | 0.78% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.76% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

