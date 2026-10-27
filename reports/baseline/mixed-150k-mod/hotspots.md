# JVM hotspots (JFR ExecutionSample)

Samples: 32045

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 3 | 3.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 6 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 8 | 2.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 11 | 1.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1463` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |
| 22 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 24 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 26 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.65% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 30 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.58% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7281` |
| 35 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7156` |
| 37 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 38 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2133` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2617` |
| 43 | 0.49% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.47% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 46 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6229` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 50 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7289` |
| 52 | 0.38% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 53 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2079` |
| 54 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 58 | 0.35% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 387` |
| 59 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.04% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 4 | 2.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1079` |
| 6 | 2.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7191` |
| 7 | 2.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 2.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4257` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7208` |
| 15 | 1.26% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 16 | 1.22% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.97% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2306` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7183` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1467` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 28 | 0.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5149` |
| 30 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

