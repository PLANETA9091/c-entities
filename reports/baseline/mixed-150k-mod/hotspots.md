# JVM hotspots (JFR ExecutionSample)

Samples: 41278

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 2 | 3.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 4 | 3.01% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 5 | 2.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 9 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 14 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.81% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8560` |
| 22 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 32 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 34 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.54% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 37 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 38 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 40 | 0.46% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 41 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 45 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 46 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 48 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 57 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.31% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.01% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 7 | 2.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8487` |
| 10 | 1.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 14 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.34% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.97% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.85% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.57% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

