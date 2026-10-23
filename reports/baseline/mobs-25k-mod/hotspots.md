# JVM hotspots (JFR ExecutionSample)

Samples: 22703

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 1.48% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.40% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 1.35% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.83% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 25 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1254` |
| 28 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 30 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 34 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 38 | 0.52% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 39 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.51% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.48% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 46 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.46% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 49 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 54 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1245` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 60 | 0.31% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 3 | 2.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.40% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 1.40% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 10 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1255` |
| 12 | 0.98% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 13 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 14 | 0.96% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 15 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 923` |
| 17 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.86% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 23 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 25 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

