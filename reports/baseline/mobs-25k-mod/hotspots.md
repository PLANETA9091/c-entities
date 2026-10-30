# JVM hotspots (JFR ExecutionSample)

Samples: 13694

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 2 | 2.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 7 | 2.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8691` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3983` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8638` |
| 16 | 1.10% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.98% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 21 | 0.96% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 25 | 0.91% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6744` |
| 26 | 0.88% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3920` |
| 27 | 0.87% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 28 | 0.85% | `java.lang.ref.Reference.get() line: 361` |
| 29 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5582` |
| 30 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5727` |
| 31 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.64% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 34 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8777` |
| 37 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 40 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 45 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.37% | `java.lang.String.hashCode() line: 2430` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4073` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8899` |
| 52 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4228` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 57 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 58 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 3 | 2.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 4 | 2.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8704` |
| 5 | 2.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 2.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 7 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.84% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.75% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 11 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.31% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 15 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 17 | 1.20% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 19 | 1.07% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 20 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 1.04% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 23 | 0.96% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.96% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 25 | 0.93% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.92% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4229` |
| 28 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 29 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 30 | 0.87% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |

GC pauses: 0, total 0 ms

