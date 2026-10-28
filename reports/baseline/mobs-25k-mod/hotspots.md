# JVM hotspots (JFR ExecutionSample)

Samples: 18157

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.30% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 8 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.53% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 17 | 1.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 18 | 0.99% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 19 | 0.92% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 20 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.79% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.68% | `java.lang.ref.Reference.get() line: 361` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7930` |
| 29 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 31 | 0.57% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6952` |
| 32 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 34 | 0.53% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.50% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 38 | 0.50% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 40 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 41 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 42 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 43 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3052` |
| 47 | 0.44% | `java.lang.String.hashCode() line: 2430` |
| 48 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5056` |
| 51 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 52 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 58 | 0.34% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 60 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 3 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 5 | 2.30% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 6 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.18% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.00% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 18 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6910` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4587` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6741` |
| 23 | 0.77% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 28 | 0.74% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.73% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 30 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

