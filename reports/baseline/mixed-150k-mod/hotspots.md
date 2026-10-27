# JVM hotspots (JFR ExecutionSample)

Samples: 40323

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 3.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 4 | 2.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 7 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 8 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 11 | 1.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.31% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 21 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 29 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 31 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.56% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 34 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.50% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.50% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 42 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 43 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 48 | 0.38% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 52 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 54 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 59 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 4 | 3.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 6 | 2.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.55% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.48% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7557` |
| 10 | 1.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 13 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 16 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.99% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 20 | 0.84% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 23 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

