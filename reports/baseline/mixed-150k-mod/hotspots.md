# JVM hotspots (JFR ExecutionSample)

Samples: 28771

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6239` |
| 2 | 3.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.45% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 4 | 2.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.32% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9897` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5471` |
| 7 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9414` |
| 14 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9842` |
| 25 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 26 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 35 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.45% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 38 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9388` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4398` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 53 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 56 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9850` |
| 59 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 60 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9502` |
| 4 | 2.79% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.34% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9433` |
| 8 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.10% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 17 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9492` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.81% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7158` |
| 23 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6249` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

