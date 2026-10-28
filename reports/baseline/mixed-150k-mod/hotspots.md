# JVM hotspots (JFR ExecutionSample)

Samples: 51130

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.57% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 6 | 1.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 9 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 11 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 13 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 18 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 20 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 23 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 24 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.53% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.48% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 32 | 0.47% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 33 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.46% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 39 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 41 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 42 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 43 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 44 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 46 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.31% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 53 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 56 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.26% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3267` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.15% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.06% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 17 | 0.92% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |
| 27 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 30 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |

GC pauses: 0, total 0 ms

