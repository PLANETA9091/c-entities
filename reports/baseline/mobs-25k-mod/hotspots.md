# JVM hotspots (JFR ExecutionSample)

Samples: 12793

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.67% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 5 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9838` |
| 10 | 1.46% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 11 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4379` |
| 16 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 1.09% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9502` |
| 20 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9363` |
| 21 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 22 | 0.93% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4316` |
| 23 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 24 | 0.85% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.84% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 26 | 0.81% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 27 | 0.78% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7265` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.57% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.52% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.52% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4469` |
| 39 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9783` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.44% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4646` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.41% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 46 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7263` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 50 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9354` |
| 53 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.31% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.32% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 2 | 3.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 3 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 6 | 2.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 2.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 2.21% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9456` |
| 13 | 1.33% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.17% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 17 | 1.16% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 1.09% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4489` |
| 20 | 1.02% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4647` |
| 21 | 1.01% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 22 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.95% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7261` |
| 24 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.92% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 28 | 0.86% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 29 | 0.84% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.81% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

GC pauses: 0, total 0 ms

