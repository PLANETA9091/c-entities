# JVM hotspots (JFR ExecutionSample)

Samples: 20835

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 5 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.19% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 10 | 1.08% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 13 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 17 | 0.86% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 19 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 21 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.73% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 23 | 0.72% | `java.lang.ref.Reference.get() line: 361` |
| 24 | 0.68% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 25 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 29 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 34 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 37 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 38 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 41 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7752` |
| 44 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 47 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6774` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 50 | 0.36% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 51 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 57 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 3 | 2.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 4 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 6 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.09% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7557` |
| 16 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 18 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.86% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.85% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.79% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 26 | 0.70% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6744` |
| 27 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.69% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.66% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |

GC pauses: 0, total 0 ms

