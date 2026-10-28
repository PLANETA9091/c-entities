# JVM hotspots (JFR ExecutionSample)

Samples: 51099

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 4 | 2.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 6 | 1.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 9 | 1.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 11 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3462` |
| 13 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 15 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 17 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 21 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 31 | 0.45% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 32 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 33 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.44% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 35 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.40% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 38 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 41 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 43 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 46 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 47 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 48 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.30% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 52 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.28% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 54 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 56 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.level() line: 5416` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 10 | 1.39% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 11 | 1.17% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.05% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.95% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 18 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.87% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.59% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5676` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.56% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.53% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.52% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

