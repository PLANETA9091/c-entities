# JVM hotspots (JFR ExecutionSample)

Samples: 32027

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 4 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 5 | 3.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 12 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.07% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.94% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.63% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 32 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 37 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 44 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 46 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 49 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 54 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 55 | 0.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 56 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 58 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 6 | 2.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 7 | 2.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.92% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8849` |
| 11 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 14 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.27% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.23% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.21% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 1.09% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.97% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.86% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6766` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

