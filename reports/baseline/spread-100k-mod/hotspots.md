# JVM hotspots (JFR ExecutionSample)

Samples: 27008

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9806` |
| 4 | 3.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 5 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9331` |
| 11 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 17 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 20 | 0.94% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 23 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.72% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9322` |
| 30 | 0.65% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 31 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 34 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 36 | 0.58% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 37 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 38 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9751` |
| 41 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2712` |
| 44 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4311` |
| 50 | 0.44% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9647` |
| 51 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7233` |
| 53 | 0.43% | `java.lang.ref.Reference.get() line: 361` |
| 54 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 55 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 59 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.37% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9424` |
| 5 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9321` |
| 8 | 1.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.45% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.40% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 14 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 19 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9414` |
| 30 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3934` |

GC pauses: 0, total 0 ms

