# JVM hotspots (JFR ExecutionSample)

Samples: 14764

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 6 | 1.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 9 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 13 | 1.16% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.02% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.87% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 24 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.65% | `java.lang.ref.Reference.get() line: 361` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 29 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 40 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3710` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3863` |
| 45 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.40% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 47 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8689` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6534` |
| 50 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4018` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 55 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1076` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3118` |
| 59 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8494` |
| 3 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 1.53% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 7 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 8 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.29% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3119` |
| 12 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 17 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 27 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 29 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

