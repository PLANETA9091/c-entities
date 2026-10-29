# JVM hotspots (JFR ExecutionSample)

Samples: 45721

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 3 | 3.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 9 | 1.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 15 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8142` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8017` |
| 39 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 45 | 0.40% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 46 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 47 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 48 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 53 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 56 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.35% | `java.lang.ref.Reference.get() line: 361` |
| 59 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 6 | 1.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.42% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8044` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8069` |
| 12 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 16 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.83% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

