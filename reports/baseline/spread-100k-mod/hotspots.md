# JVM hotspots (JFR ExecutionSample)

Samples: 32724

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.64% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.72% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8965` |
| 3 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 4 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 6 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 10 | 1.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5715` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5133` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.84% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.76% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 31 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8910` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8785` |
| 39 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6877` |
| 47 | 0.42% | `java.lang.ref.Reference.get() line: 361` |
| 48 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8820` |
| 7 | 2.51% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.91% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.66% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8812` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5170` |
| 16 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 18 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.91% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |

GC pauses: 0, total 0 ms

