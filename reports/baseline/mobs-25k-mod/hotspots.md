# JVM hotspots (JFR ExecutionSample)

Samples: 20804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.26% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 6 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 8 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 12 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.96% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.91% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 17 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 20 | 0.73% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 22 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.61% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 24 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.54% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 26 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.49% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 30 | 0.49% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 31 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 33 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 35 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 39 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 40 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 45 | 0.36% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 46 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.33% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 50 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.32% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 7003` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 56 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 58 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.27% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 7 | 1.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.97% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 13 | 0.96% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 16 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 19 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.78% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6973` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4638` |

GC pauses: 0, total 0 ms

