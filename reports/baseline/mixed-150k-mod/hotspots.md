# JVM hotspots (JFR ExecutionSample)

Samples: 32145

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 2 | 3.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 4 | 3.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 5 | 3.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 9 | 2.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 11 | 1.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 12 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.69% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 32 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.57% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 38 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 40 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.33% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 55 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6545` |
| 57 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.30% | `java.lang.ref.Reference.get() line: 361` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 6 | 2.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.09% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 2.06% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.96% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.93% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 13 | 1.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 15 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 17 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

