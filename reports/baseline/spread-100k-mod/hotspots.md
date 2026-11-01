# JVM hotspots (JFR ExecutionSample)

Samples: 31668

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9080` |
| 3 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 6 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 11 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5808` |
| 14 | 1.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.02% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5226` |
| 21 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.85% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.79% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.65% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 32 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.60% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3819` |
| 37 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4165` |
| 38 | 0.54% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 39 | 0.54% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 46 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 47 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8900` |
| 53 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4102` |
| 54 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 55 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 58 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6992` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5975` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8935` |
| 6 | 2.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.13% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.83% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.26% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8927` |
| 22 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.95% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 24 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

