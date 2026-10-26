# JVM hotspots (JFR ExecutionSample)

Samples: 15620

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |
| 4 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 6 | 1.66% | `java.util.EnumMap.get(Object) line: 248` |
| 7 | 1.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 11 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 16 | 1.04% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 1.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 1.00% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 20 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 21 | 0.94% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 22 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3627` |
| 23 | 0.91% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.81% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.80% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.74% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3959` |
| 31 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 35 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 38 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 40 | 0.63% | `java.util.ArrayDeque.size() line: 647` |
| 41 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 42 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 43 | 0.58% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.58% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3993` |
| 45 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3655` |
| 47 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 48 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 49 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 50 | 0.51% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1170` |
| 51 | 0.50% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.48% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3948` |
| 54 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 55 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 57 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 58 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 59 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 4 | 2.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.94% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |
| 8 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 14 | 1.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4143` |
| 16 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4173` |
| 18 | 1.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.04% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 20 | 1.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4087` |
| 23 | 0.84% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 24 | 0.83% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.81% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 27 | 0.80% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 28 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.73% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms

