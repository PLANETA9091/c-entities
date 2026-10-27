# JVM hotspots (JFR ExecutionSample)

Samples: 33126

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 3 | 3.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 8 | 2.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 10 | 1.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 12 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 16 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 23 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 29 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.57% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 32 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.56% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 43 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 45 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 50 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 53 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.37% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 55 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.30% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 3 | 3.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 7 | 2.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.05% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 12 | 1.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 14 | 1.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 16 | 1.22% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 24 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5416` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

