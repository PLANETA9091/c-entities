# JVM hotspots (JFR ExecutionSample)

Samples: 19441

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.08% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.28% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 17 | 0.84% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 19 | 0.74% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.73% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 23 | 0.64% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 24 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 26 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 30 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 32 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 34 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 36 | 0.53% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7752` |
| 37 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6774` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2884` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 49 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 52 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 55 | 0.35% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 3 | 3.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 4 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.25% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.59% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.29% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 17 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.87% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 19 | 0.85% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.83% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 21 | 0.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6563` |
| 24 | 0.74% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4409` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6732` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms

