# JVM hotspots (JFR ExecutionSample)

Samples: 36294

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 5 | 2.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 12 | 1.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 14 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7905` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.63% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.57% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 39 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 41 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.46% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7462` |
| 46 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4884` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3202` |
| 54 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.34% | `java.lang.ref.Reference.get() line: 361` |
| 58 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.33% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 6 | 2.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.88% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 1.86% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.76% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 12 | 1.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 16 | 1.13% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 30 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |

GC pauses: 0, total 0 ms

