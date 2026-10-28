# JVM hotspots (JFR ExecutionSample)

Samples: 55695

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 4 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 6 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4550` |
| 16 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7721` |
| 19 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1876` |
| 22 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.64% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.53% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 39 | 0.39% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6669` |
| 40 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7596` |
| 41 | 0.38% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 42 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1860` |
| 49 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 50 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2682` |
| 55 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.29% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 58 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 60 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7631` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1399` |
| 9 | 1.63% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.09% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7623` |
| 14 | 1.01% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 16 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1861` |
| 21 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.64% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 25 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 29 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

