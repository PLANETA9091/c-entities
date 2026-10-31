# JVM hotspots (JFR ExecutionSample)

Samples: 31706

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.74% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 5 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 6 | 2.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.57% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 8 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 2.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 12 | 1.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 13 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 17 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.89% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 26 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 30 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 34 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 36 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 38 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 41 | 0.44% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 42 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 44 | 0.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 45 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 49 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 6 | 2.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.12% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.06% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 13 | 1.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.41% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.29% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 1.17% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 21 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.91% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.85% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 29 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |

GC pauses: 0, total 0 ms

