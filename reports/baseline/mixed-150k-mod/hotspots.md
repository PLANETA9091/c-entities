# JVM hotspots (JFR ExecutionSample)

Samples: 64275

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 2.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 8 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 13 | 1.03% | `java.lang.ref.Reference.get() line: 361` |
| 14 | 0.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 16 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 23 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.53% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 31 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 32 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 41 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 42 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 47 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 52 | 0.29% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.28% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 57 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 13 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 16 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 20 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.63% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 27 | 0.55% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |
| 30 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

