# JVM hotspots (JFR ExecutionSample)

Samples: 39423

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 3 | 2.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 4 | 2.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 13 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 18 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.58% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.53% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.51% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 34 | 0.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 35 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 39 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4281` |
| 47 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 50 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 51 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 54 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 3 | 2.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 9 | 1.39% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 13 | 1.23% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 17 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.72% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5170` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.61% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

