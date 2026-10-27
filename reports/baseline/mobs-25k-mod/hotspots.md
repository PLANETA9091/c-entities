# JVM hotspots (JFR ExecutionSample)

Samples: 19573

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 2 | 4.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 7 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 11 | 1.08% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 13 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 14 | 0.82% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 15 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 16 | 0.78% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.75% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.73% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 20 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 21 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 25 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7424` |
| 27 | 0.52% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 28 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 30 | 0.49% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 31 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.47% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 34 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 36 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 42 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 48 | 0.35% | `java.lang.ThreadLocal.get() line: 171` |
| 49 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6446` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2619` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 54 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.32% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 56 | 0.32% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 57 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 59 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.29% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.12% | `java.util.HashMap.containsKey(Object) line: 603` |
| 2 | 4.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 4 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 0.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 14 | 0.89% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 15 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 17 | 0.78% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 18 | 0.75% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 19 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.73% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 22 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.64% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6235` |
| 26 | 0.64% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6416` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4081` |
| 28 | 0.63% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |

GC pauses: 0, total 0 ms

