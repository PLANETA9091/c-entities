# JVM hotspots (JFR ExecutionSample)

Samples: 22311

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 6 | 1.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.25% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 13 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.04% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 17 | 0.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 19 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 21 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 23 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.66% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.64% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.60% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 29 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2619` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7424` |
| 32 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2556` |
| 36 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 41 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 43 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.42% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 46 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 47 | 0.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5293` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 50 | 0.39% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 51 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2729` |
| 55 | 0.37% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 56 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 58 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 4 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 1.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 6 | 1.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.17% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.04% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 19 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.79% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.77% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.71% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6235` |
| 26 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 2863` |

GC pauses: 0, total 0 ms

