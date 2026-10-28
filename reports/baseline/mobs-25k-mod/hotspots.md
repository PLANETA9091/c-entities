# JVM hotspots (JFR ExecutionSample)

Samples: 22017

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.86% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 9 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 15 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.00% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 18 | 0.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.79% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 21 | 0.73% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.71% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 23 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 27 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 32 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 35 | 0.50% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 36 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 37 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.44% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 43 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 45 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 47 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 48 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 50 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 53 | 0.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 54 | 0.38% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5850` |
| 55 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 56 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 57 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 2.38% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.01% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.80% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 20 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.71% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.62% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4638` |

GC pauses: 0, total 0 ms

