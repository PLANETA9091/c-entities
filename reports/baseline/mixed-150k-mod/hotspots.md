# JVM hotspots (JFR ExecutionSample)

Samples: 52332

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 4 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 11 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 15 | 0.99% | `java.lang.ref.Reference.get() line: 361` |
| 16 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 18 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 22 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 35 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 41 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.41% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 43 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 52 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 54 | 0.32% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 55 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 60 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 12 | 1.11% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 14 | 1.07% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

