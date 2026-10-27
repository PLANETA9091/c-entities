# JVM hotspots (JFR ExecutionSample)

Samples: 17139

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.85% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 4 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 8 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 11 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 0.90% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6210` |
| 15 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 19 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 21 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.64% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 23 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 25 | 0.54% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 33 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.46% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 35 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 37 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.43% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6406` |
| 40 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 43 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.40% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 46 | 0.40% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 47 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 51 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 53 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2598` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2708` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4510` |
| 57 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7384` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 3 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.86% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 5 | 1.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 10 | 1.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 0.95% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7189` |
| 15 | 0.84% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 16 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 17 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 25 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6193` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 28 | 0.67% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

