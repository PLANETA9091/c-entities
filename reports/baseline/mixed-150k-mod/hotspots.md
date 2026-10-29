# JVM hotspots (JFR ExecutionSample)

Samples: 31418

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 3 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 5 | 2.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 9 | 2.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 11 | 2.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.84% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 13 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 20 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 1.02% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.95% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 26 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 32 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.69% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 34 | 0.67% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.57% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 39 | 0.50% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 44 | 0.44% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5283` |
| 53 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 54 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 5 | 2.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 2.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.09% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8255` |
| 12 | 1.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 13 | 1.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.45% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.34% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 1.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 1.03% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 27 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3168` |
| 30 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

