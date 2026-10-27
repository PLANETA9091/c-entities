# JVM hotspots (JFR ExecutionSample)

Samples: 34425

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7652` |
| 3 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 9 | 1.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4426` |
| 11 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |
| 16 | 1.18% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1745` |
| 19 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.08% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 26 | 0.74% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 27 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.61% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 34 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 36 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.56% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7597` |
| 38 | 0.55% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2430` |
| 41 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.47% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 43 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 45 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6545` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2914` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 52 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 55 | 0.35% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 58 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2376` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 4 | 3.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 5 | 2.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7507` |
| 8 | 2.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.72% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4573` |
| 12 | 1.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.38% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 14 | 1.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7524` |
| 16 | 1.19% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1749` |
| 20 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7499` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.74% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5465` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2306` |
| 30 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

