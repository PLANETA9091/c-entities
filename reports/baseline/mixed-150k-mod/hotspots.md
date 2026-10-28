# JVM hotspots (JFR ExecutionSample)

Samples: 43641

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 5 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 6 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 8 | 1.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 10 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 13 | 1.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.64% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 32 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 33 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.51% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 35 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 38 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 41 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 43 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 46 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 51 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 52 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 54 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.30% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.28% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.24% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.51% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.09% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 30 | 0.64% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |

GC pauses: 0, total 0 ms

