# JVM hotspots (JFR ExecutionSample)

Samples: 14890

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 7 | 1.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 10 | 1.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 12 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.01% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.97% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 21 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.82% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 23 | 0.81% | `java.lang.ref.Reference.get() line: 361` |
| 24 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.73% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.65% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3808` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3745` |
| 35 | 0.57% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8724` |
| 36 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6569` |
| 38 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 44 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3898` |
| 49 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8602` |
| 53 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 58 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.32% | `java.lang.ThreadLocal.get() line: 171` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8529` |
| 3 | 1.95% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 4 | 1.81% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3154` |
| 5 | 1.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 10 | 1.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.12% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 1.05% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8512` |
| 20 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 22 | 0.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5357` |
| 23 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3918` |
| 27 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 28 | 0.79% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.73% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

