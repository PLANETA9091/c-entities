# JVM hotspots (JFR ExecutionSample)

Samples: 37974

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 2.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 7 | 1.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 11 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.13% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 25 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 31 | 0.54% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 32 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.53% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 34 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.47% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 39 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.43% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 42 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 43 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 52 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 54 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 59 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.30% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.61% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 3 | 3.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.60% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 2.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 7 | 1.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.40% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.27% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 21 | 0.75% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 22 | 0.73% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5498` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 516` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

