# JVM hotspots (JFR ExecutionSample)

Samples: 33838

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 4.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 3 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1331` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 6 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 8 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3359` |
| 14 | 1.18% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.01% | `java.util.HashMap.getNode(Object) line: 586` |
| 17 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.92% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 25 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.80% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 27 | 0.80% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3365` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3362` |
| 30 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 31 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3375` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 35 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2821` |
| 37 | 0.59% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 38 | 0.57% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 39 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.54% | `java.util.HashMap$HashIterator.nextNode() line: 1605` |
| 41 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 42 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 44 | 0.49% | `java.util.ArrayDeque.elementAt(Object[], int) line: 257` |
| 45 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 46 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 48 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5130` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 53 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3147` |
| 55 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 57 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.38% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7458` |
| 59 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8250` |
| 60 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 2 | 4.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3380` |
| 3 | 4.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.12% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 5 | 2.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |
| 9 | 1.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.25% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 13 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 16 | 1.01% | `java.util.HashMap.get(Object) line: 565` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 19 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 21 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 425` |
| 23 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8126` |
| 28 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |

GC pauses: 0, total 0 ms

