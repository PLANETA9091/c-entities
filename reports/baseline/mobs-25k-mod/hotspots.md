# JVM hotspots (JFR ExecutionSample)

Samples: 54128

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 3.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.52% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 7 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5273` |
| 18 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.canCollideWithBukkit$(LivingEntity, Entity) line: 1621` |
| 21 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 24 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8237` |
| 26 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.push(Entity) line: 2472` |
| 31 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 32 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 35 | 0.35% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 38 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 45 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 46 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8445` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8184` |
| 49 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 52 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.canCollideWithBukkit$(LivingEntity, Entity) line: 1620` |
| 54 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 55 | 0.22% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8323` |
| 57 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 60 | 0.20% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 6 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.94% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.75% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity)` |
| 9 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8250` |
| 11 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.66% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.50% | `net.minecraft.world.entity.LivingEntity.push(Entity) line: 2948` |
| 16 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.48% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 21 | 0.46% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 24 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 25 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

