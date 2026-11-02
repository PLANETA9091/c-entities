# JVM hotspots (JFR ExecutionSample)

Samples: 9417

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `java.lang.ref.Reference.get() line: 361` |
| 2 | 2.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 4 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10097` |
| 6 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 10 | 1.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4400` |
| 13 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4337` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9499` |
| 16 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7353` |
| 19 | 1.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6743` |
| 22 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.84% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10009` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6421` |
| 25 | 0.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 27 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.63% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 29 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6587` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4667` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4490` |
| 39 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.48% | `java.lang.String.hashCode() line: 2430` |
| 41 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9652` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6464` |
| 45 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.35% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.firstOf(Set) line: 6717` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7351` |
| 56 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 58 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6826` |
| 3 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 2.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 2.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 9 | 1.88% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6432` |
| 10 | 1.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9652` |
| 13 | 1.38% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4668` |
| 15 | 1.26% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4510` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7349` |
| 18 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 1.01% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 20 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6742` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 24 | 0.85% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6466` |
| 25 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6899` |
| 28 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

