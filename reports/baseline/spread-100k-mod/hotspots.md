# JVM hotspots (JFR ExecutionSample)

Samples: 30543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 7 | 2.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 9 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 13 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 18 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.81% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2448` |
| 34 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3766` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.48% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.48% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6527` |
| 42 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 43 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 45 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.42% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 48 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 50 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3703` |
| 54 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8560` |
| 55 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 57 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 5 | 2.49% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.41% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.85% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.63% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.41% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.16% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 21 | 1.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 22 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.93% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.86% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.84% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 30 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

