# JVM hotspots (JFR ExecutionSample)

Samples: 22467

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.29% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 6 | 1.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 10 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 13 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 17 | 0.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.84% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 19 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.69% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.69% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 22 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 25 | 0.63% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7930` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 28 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.55% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 31 | 0.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 32 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.52% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 34 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 39 | 0.44% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 43 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6952` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 48 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 50 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1805` |
| 55 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 56 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 57 | 0.34% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 59 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.isNullNibbleVisible() line: 280` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 3 | 3.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 4 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 15 | 0.94% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 16 | 0.88% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 0.84% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 18 | 0.74% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4587` |
| 22 | 0.69% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 24 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map) line: 134` |
| 27 | 0.64% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 28 | 0.63% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6910` |
| 30 | 0.57% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms

