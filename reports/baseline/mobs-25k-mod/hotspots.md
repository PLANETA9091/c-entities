# JVM hotspots (JFR ExecutionSample)

Samples: 16084

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 5 | 2.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 7 | 1.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 9 | 1.41% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 12 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 17 | 0.91% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 20 | 0.83% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 22 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 24 | 0.76% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 25 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.71% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 30 | 0.67% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.67% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.59% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 35 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 36 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 38 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 41 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.51% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5850` |
| 43 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 44 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.50% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 46 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3094` |
| 47 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.47% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 51 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 53 | 0.40% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6536` |
| 54 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 56 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5107` |
| 59 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7002` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 4 | 2.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 7 | 1.52% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 8 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6961` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 18 | 0.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 19 | 0.88% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 21 | 0.84% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.76% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.73% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.68% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

