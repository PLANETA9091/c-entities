# JVM hotspots (JFR ExecutionSample)

Samples: 13794

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 9 | 1.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8699` |
| 10 | 1.22% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5449` |
| 13 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.07% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 15 | 1.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 18 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.80% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 22 | 0.72% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 25 | 0.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.62% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3850` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3787` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6611` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8644` |
| 36 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.46% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.39% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 47 | 0.39% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 48 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4095` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 51 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 53 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 56 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.26% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 2 | 2.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 2.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 5 | 1.87% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8554` |
| 10 | 1.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.26% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 15 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 1.06% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 1.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 23 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 27 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3960` |
| 29 | 0.80% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4096` |
| 30 | 0.78% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |

GC pauses: 0, total 0 ms

