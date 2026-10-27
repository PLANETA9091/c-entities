# JVM hotspots (JFR ExecutionSample)

Samples: 17094

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 9 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 12 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1759` |
| 16 | 0.98% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 17 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.86% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 20 | 0.84% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 22 | 0.75% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6559` |
| 25 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 26 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.63% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 29 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 30 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7666` |
| 32 | 0.53% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7733` |
| 33 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2928` |
| 35 | 0.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4440` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6755` |
| 38 | 0.49% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 39 | 0.49% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.45% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2865` |
| 45 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.44% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 48 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 50 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7611` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4859` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1762` |
| 53 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 54 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 56 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6288` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 60 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 3 | 3.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1763` |
| 5 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7538` |
| 8 | 1.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.13% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 11 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7521` |
| 18 | 0.86% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 19 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.84% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.77% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6713` |
| 25 | 0.74% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 28 | 0.66% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6544` |
| 30 | 0.63% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |

GC pauses: 0, total 0 ms

