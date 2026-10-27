# JVM hotspots (JFR ExecutionSample)

Samples: 32695

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.33% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 3 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 6 | 2.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 7 | 2.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 10 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 12 | 1.56% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 21 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 25 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.69% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 30 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.54% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 35 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.51% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2599` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 41 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 42 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 46 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7270` |
| 48 | 0.41% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6210` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 53 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 626` |
| 58 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4241` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.34% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.18% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 5 | 2.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 6 | 2.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.12% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 12 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.36% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 16 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7189` |
| 17 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5130` |
| 25 | 0.78% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 26 | 0.78% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.69% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |

GC pauses: 0, total 0 ms

