# JVM hotspots (JFR ExecutionSample)

Samples: 57379

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 2.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 6 | 2.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 14 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 16 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 17 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.59% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 26 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 28 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 31 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 32 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 36 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 41 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 46 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 49 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 54 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.30% | `java.lang.String.equals(Object) line: 1926` |
| 57 | 0.30% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 2.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.43% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.17% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 17 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.63% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.59% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.56% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

