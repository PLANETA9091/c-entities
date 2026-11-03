# JVM hotspots (JFR ExecutionSample)

Samples: 9804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 3.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 4 | 2.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 7 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.99% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 12 | 1.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |
| 16 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 18 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 22 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 24 | 0.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 26 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 28 | 0.66% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 29 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.60% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.59% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.56% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.firstOf(Set) line: 6954` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.44% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 41 | 0.44% | `net.minecraft.core.component.PatchedDataComponentMap.equals(Object) line: 222` |
| 42 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.42% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6673` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6647` |
| 58 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 4.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 3 | 3.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 4 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.15% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 2.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 2.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 10 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 12 | 1.34% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 13 | 1.23% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7586` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7136` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6979` |
| 17 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.06% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6978` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6808` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.66% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |
| 30 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

