# JVM hotspots (JFR ExecutionSample)

Samples: 17595

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 8 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 10 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 12 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 20 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.58% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8250` |
| 28 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.50% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 31 | 0.47% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 32 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2176` |
| 35 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.41% | `java.lang.ref.Reference.get() line: 361` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3401` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6095` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 49 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 50 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8128` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2179` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 60 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 3 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2180` |
| 8 | 1.40% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8055` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8030` |
| 13 | 1.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.01% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.99% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4883` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.89% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.71% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

