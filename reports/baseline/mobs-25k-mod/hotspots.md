# JVM hotspots (JFR ExecutionSample)

Samples: 17186

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.81% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 3 | 1.73% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 4539` |
| 4 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 8 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4510` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 14 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4373` |
| 17 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.83% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.77% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 21 | 0.76% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 27 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.67% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 30 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 32 | 0.59% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 36 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 37 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3838` |
| 38 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.49% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 45 | 0.42% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 46 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 56 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3827` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.73% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 1.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 8 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.23% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 11 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4010` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 16 | 1.11% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4022` |
| 18 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4052` |
| 20 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 22 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 26 | 0.77% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 27 | 0.76% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 28 | 0.75% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

