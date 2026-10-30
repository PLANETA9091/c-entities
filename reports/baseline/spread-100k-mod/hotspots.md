# JVM hotspots (JFR ExecutionSample)

Samples: 30923

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.94% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 4 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 5 | 2.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 8 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 13 | 1.80% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 1.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 18 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 25 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.82% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.68% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 36 | 0.60% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.57% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 38 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3773` |
| 43 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 46 | 0.47% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |
| 48 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6534` |
| 49 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8567` |
| 50 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 53 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 55 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 56 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8442` |
| 58 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8477` |
| 3 | 2.94% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.37% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.83% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.80% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 12 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.31% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 1.03% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4827` |
| 24 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.92% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8469` |
| 27 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.82% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |

GC pauses: 0, total 0 ms

