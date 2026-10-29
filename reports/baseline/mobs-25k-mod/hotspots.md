# JVM hotspots (JFR ExecutionSample)

Samples: 17475

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 7 | 2.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 9 | 1.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 14 | 1.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 15 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 18 | 1.10% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 19 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.98% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.89% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 27 | 0.76% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 28 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 30 | 0.68% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 31 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 32 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.61% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5896` |
| 35 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3202` |
| 37 | 0.58% | `java.lang.ref.Reference.get() line: 361` |
| 38 | 0.54% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 39 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 40 | 0.52% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 42 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8027` |
| 44 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 46 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.41% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3139` |
| 53 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 55 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7905` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 4 | 2.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 5 | 2.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 13 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 16 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 21 | 0.90% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 22 | 0.90% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.88% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 26 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.84% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.80% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5892` |
| 30 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms

