# JVM hotspots (JFR ExecutionSample)

Samples: 13865

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 11 | 1.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.23% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 18 | 0.95% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 20 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.73% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.66% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 32 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.61% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.61% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 2980` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 37 | 0.56% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8490` |
| 38 | 0.54% | `java.lang.ref.Reference.get() line: 361` |
| 39 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6335` |
| 46 | 0.40% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 441` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2339` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 50 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8368` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5323` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3725` |
| 57 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 647` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 60 | 0.29% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8295` |
| 3 | 2.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.01% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 2981` |
| 5 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.58% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 7 | 1.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 11 | 1.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 17 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 0.95% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 19 | 0.94% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.94% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1831` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 25 | 0.82% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 28 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 30 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

