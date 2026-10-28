# JVM hotspots (JFR ExecutionSample)

Samples: 21497

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1876` |
| 5 | 1.69% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7843` |
| 7 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1857` |
| 9 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 14 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.81% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6669` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 19 | 0.70% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 20 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.64% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1860` |
| 23 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 25 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4550` |
| 28 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.49% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 30 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7721` |
| 33 | 0.44% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 34 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 36 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 40 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 41 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 43 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.37% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3028` |
| 49 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.isNullNibbleVisible() line: 280` |
| 52 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3138` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2965` |
| 56 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 57 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4969` |
| 59 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1854` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1399` |
| 3 | 2.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1861` |
| 4 | 1.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 1.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7648` |
| 11 | 1.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.85% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 17 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 18 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.78% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6835` |
| 23 | 0.71% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 24 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.70% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3272` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4500` |
| 28 | 0.62% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 29 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7623` |

GC pauses: 0, total 0 ms

