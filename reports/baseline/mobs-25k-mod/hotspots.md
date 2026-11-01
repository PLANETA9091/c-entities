# JVM hotspots (JFR ExecutionSample)

Samples: 15095

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 6 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.59% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 11 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 15 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 16 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.91% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.76% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.75% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.63% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9128` |
| 33 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.55% | `java.lang.ref.Reference.get() line: 361` |
| 36 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.53% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3441` |
| 39 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.46% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 44 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 45 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 46 | 0.43% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 47 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 51 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.34% | `java.lang.ThreadLocal.get() line: 171` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6973` |
| 55 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 57 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9006` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8933` |
| 4 | 1.91% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3442` |
| 5 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 7 | 1.62% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 8 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 9 | 1.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 12 | 1.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.16% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.03% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 21 | 0.91% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 22 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.85% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 26 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 28 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.76% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |

GC pauses: 0, total 0 ms

