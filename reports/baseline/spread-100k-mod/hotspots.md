# JVM hotspots (JFR ExecutionSample)

Samples: 32735

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9345` |
| 4 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 7 | 2.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 10 | 1.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 14 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 19 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.93% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.78% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 31 | 0.68% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4017` |
| 33 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.58% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4363` |
| 39 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 41 | 0.53% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 42 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 44 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9163` |
| 46 | 0.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 47 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.44% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 54 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2701` |
| 55 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.38% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7193` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 59 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4300` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9200` |
| 6 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.04% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.83% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.29% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.28% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.20% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5462` |
| 20 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9192` |
| 21 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 1.08% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2083` |
| 26 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.86% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

