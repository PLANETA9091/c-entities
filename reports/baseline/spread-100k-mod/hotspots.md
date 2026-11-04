# JVM hotspots (JFR ExecutionSample)

Samples: 32939

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 2 | 4.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 5 | 2.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 6 | 2.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2669` |
| 7 | 2.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10723` |
| 8 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 10 | 1.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2400` |
| 15 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.99% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.98% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 24 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.75% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 27 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7249` |
| 28 | 0.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 29 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7317` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 35 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 36 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.47% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7859` |
| 40 | 0.46% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3111` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 44 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3178` |
| 46 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 47 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.39% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7093` |
| 55 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10531` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6755` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 2 | 5.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.32% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 5 | 2.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2442` |
| 7 | 2.17% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.83% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7332` |
| 12 | 1.49% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7317` |
| 16 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 17 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 24 | 0.64% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 26 | 0.63% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.60% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 30 | 0.59% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |

GC pauses: 0, total 0 ms

