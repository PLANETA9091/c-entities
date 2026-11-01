# JVM hotspots (JFR ExecutionSample)

Samples: 32720

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5987` |
| 7 | 2.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 8 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5833` |
| 12 | 1.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5251` |
| 18 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.05% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 33 | 0.61% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3844` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 36 | 0.59% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4190` |
| 40 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.52% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.51% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 45 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7017` |
| 47 | 0.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 48 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 52 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 53 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8925` |
| 54 | 0.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 55 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3717` |
| 60 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8960` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 2.10% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.82% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 19 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8952` |
| 27 | 0.89% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.88% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

