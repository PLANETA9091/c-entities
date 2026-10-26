# JVM hotspots (JFR ExecutionSample)

Samples: 16160

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6354` |
| 4 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.51% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6320` |
| 13 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.gsMirrorTickRunning(GoalSelector, boolean) line: 4745` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 17 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.93% | `java.lang.String.hashCode() line: 2435` |
| 19 | 0.89% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6166` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4679` |
| 22 | 0.88% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 23 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3921` |
| 28 | 0.75% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1170` |
| 30 | 0.65% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 33 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 34 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 35 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 36 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 38 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 39 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 41 | 0.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4670` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.43% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 53 | 0.41% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4678` |
| 54 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4694` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 2.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4519` |
| 4 | 2.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 7 | 2.37% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 10 | 1.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 4650` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4678` |
| 17 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.handleRelativeFrictionAndCalculateMovement(Vec3, float) line: 3303` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 23 | 0.89% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.gsMirrorTickRunning(GoalSelector, boolean) line: 4745` |
| 26 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 27 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.74% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 29 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

