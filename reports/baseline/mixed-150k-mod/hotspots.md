# JVM hotspots (JFR ExecutionSample)

Samples: 38636

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 3 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8205` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4382` |
| 11 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4955` |
| 14 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.65% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5102` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8150` |
| 42 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 45 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3077` |
| 47 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.39% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 49 | 0.39% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 50 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 55 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.33% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2209` |
| 60 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8060` |
| 5 | 2.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 14 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8077` |
| 16 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 25 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.63% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

