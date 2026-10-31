# JVM hotspots (JFR ExecutionSample)

Samples: 31444

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 4 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 10 | 1.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 19 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 24 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.76% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 28 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 31 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 34 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 38 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 39 | 0.55% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 40 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 42 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 43 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 46 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 49 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6889` |
| 52 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 54 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5877` |
| 57 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.39% | `java.lang.ref.Reference.get() line: 361` |
| 59 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 5 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.27% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.97% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.85% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.37% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.29% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.07% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 23 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 25 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 26 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.91% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.82% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

