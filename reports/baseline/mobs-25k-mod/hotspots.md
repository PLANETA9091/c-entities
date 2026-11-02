# JVM hotspots (JFR ExecutionSample)

Samples: 9685

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9897` |
| 8 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 11 | 1.31% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6239` |
| 13 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9414` |
| 16 | 1.01% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4398` |
| 18 | 0.95% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.94% | `java.lang.ref.Reference.get() line: 361` |
| 20 | 0.92% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4335` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.66% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 26 | 0.64% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4665` |
| 29 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7282` |
| 31 | 0.59% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 32 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.57% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 37 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 42 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4488` |
| 46 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 56 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.35% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 59 | 0.35% | `net.minecraft.world.entity.Entity.getDeltaMovement() line: 5074` |
| 60 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.05% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 4 | 1.95% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 1.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 9 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9502` |
| 17 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.10% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3576` |
| 19 | 1.05% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4666` |
| 20 | 1.03% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 21 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4508` |
| 24 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.85% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

