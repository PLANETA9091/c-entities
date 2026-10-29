# JVM hotspots (JFR ExecutionSample)

Samples: 17509

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 7 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 8 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 11 | 1.16% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 19 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.77% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 23 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 24 | 0.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.66% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8044` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 32 | 0.59% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 36 | 0.50% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5913` |
| 39 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 40 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 43 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3219` |
| 47 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 50 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 53 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 59 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 1.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 4 | 1.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 9 | 1.27% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7849` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.10% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 16 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 0.94% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 18 | 0.93% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 19 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.82% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4701` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.67% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

