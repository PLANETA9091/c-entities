# JVM hotspots (JFR ExecutionSample)

Samples: 43367

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 3 | 2.81% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8013` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 5 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4214` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 10 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4787` |
| 13 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7958` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.59% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7515` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7833` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.43% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.38% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 47 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 52 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 55 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8080` |
| 60 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7868` |
| 7 | 2.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7885` |
| 11 | 1.33% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7860` |
| 13 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1577` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 27 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4251` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

