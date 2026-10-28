# JVM hotspots (JFR ExecutionSample)

Samples: 45883

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 6 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 8 | 1.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 12 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.05% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 20 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.58% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 33 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 36 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 41 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 45 | 0.38% | `java.lang.ref.Reference.get() line: 361` |
| 46 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 49 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 50 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.32% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 53 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 55 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7816` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 59 | 0.29% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 10 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.07% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 20 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |

GC pauses: 0, total 0 ms

