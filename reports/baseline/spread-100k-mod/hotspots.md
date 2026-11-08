# JVM hotspots (JFR ExecutionSample)

Samples: 36037

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 6.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 4 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 6 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 8 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 11 | 1.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 13 | 1.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.87% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3327` |
| 22 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 28 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 31 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.68% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 33 | 0.65% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 34 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 35 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3324` |
| 36 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 38 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 41 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3414` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 44 | 0.46% | `java.util.ArrayList.elementData(int) line: 412` |
| 45 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3323` |
| 47 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 51 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 441` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 54 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 55 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 59 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3102` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 2 | 8.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 3 | 4.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 5 | 2.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 6 | 2.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.52% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 10 | 1.46% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 11 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 18 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.88% | `java.util.HashMap.get(Object) line: 565` |
| 20 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 23 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3323` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.71% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 30 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms

