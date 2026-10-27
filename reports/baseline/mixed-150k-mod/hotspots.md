# JVM hotspots (JFR ExecutionSample)

Samples: 32704

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 5 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 7 | 2.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 13 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.22% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.06% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 20 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 28 | 0.62% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 31 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 36 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 39 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 43 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.41% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 45 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.41% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 47 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 49 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.34% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2619` |
| 58 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6446` |
| 60 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 3 | 3.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 7 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.01% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 13 | 1.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 15 | 1.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 17 | 1.06% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5170` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.80% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

