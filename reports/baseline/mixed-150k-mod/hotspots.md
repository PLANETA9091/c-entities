# JVM hotspots (JFR ExecutionSample)

Samples: 44662

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 3 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 5 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.60% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 10 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 18 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8128` |
| 28 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 33 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 35 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 37 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 41 | 0.45% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 42 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.38% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5083` |
| 49 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.21% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 7 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8055` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8030` |
| 12 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 17 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.61% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

