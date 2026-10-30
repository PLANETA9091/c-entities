# JVM hotspots (JFR ExecutionSample)

Samples: 12849

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 8 | 1.67% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 9 | 1.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 12 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 14 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 16 | 1.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 18 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 21 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.93% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3766` |
| 23 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.77% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 28 | 0.73% | `java.lang.ref.Reference.get() line: 361` |
| 29 | 0.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6527` |
| 31 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 33 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.54% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3703` |
| 40 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8682` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8560` |
| 50 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3856` |
| 55 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4011` |
| 57 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.30% | `java.util.ImmutableCollections.listCopy(Collection) line: 191` |
| 59 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8487` |
| 3 | 1.84% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3112` |
| 4 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 5 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 15 | 1.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 17 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 1.08% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.90% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.90% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4012` |
| 27 | 0.86% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 28 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 29 | 0.86% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |

GC pauses: 0, total 0 ms

