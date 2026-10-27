# JVM hotspots (JFR ExecutionSample)

Samples: 41166

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 3 | 2.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 5 | 2.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 9 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 14 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 18 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 20 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.58% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.57% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 33 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 35 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 36 | 0.51% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 37 | 0.46% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 38 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 40 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4281` |
| 44 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 45 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 47 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 50 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 3 | 3.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 9 | 1.44% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 11 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.34% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.15% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.07% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.65% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.61% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5170` |

GC pauses: 0, total 0 ms

