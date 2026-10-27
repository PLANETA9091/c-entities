# JVM hotspots (JFR ExecutionSample)

Samples: 18580

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 9 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 10 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.95% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 17 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 19 | 0.76% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 22 | 0.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 23 | 0.73% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |
| 25 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 31 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7670` |
| 37 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5539` |
| 41 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 44 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 46 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 49 | 0.40% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 52 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 54 | 0.37% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2975` |
| 58 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 60 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 4 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 1.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 6 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 11 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 12 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 0.95% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.90% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 18 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.81% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4327` |
| 22 | 0.77% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 23 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.64% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3109` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

