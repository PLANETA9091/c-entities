# JVM hotspots (JFR ExecutionSample)

Samples: 56948

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.00% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 0.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 0.96% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 10 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 18 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 32 | 0.35% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 33 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 34 | 0.33% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 35 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 38 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.30% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 40 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 51 | 0.26% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 52 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.24% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 55 | 0.24% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 57 | 0.24% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 58 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 60 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.17% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 6 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.87% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.55% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 15 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 22 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.44% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 25 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.41% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.38% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.38% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |

GC pauses: 0, total 0 ms

