# JVM hotspots (JFR ExecutionSample)

Samples: 39751

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 4 | 2.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 5 | 1.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3905` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.80% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 25 | 0.76% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.62% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 32 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.60% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.56% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7640` |
| 36 | 0.55% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 38 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 40 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4616` |
| 41 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.50% | `java.lang.ref.Reference.get() line: 361` |
| 44 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 45 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 46 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 51 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1470` |
| 54 | 0.40% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.40% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 56 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 60 | 0.36% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7550` |
| 4 | 2.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.84% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 9 | 1.61% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3942` |
| 13 | 1.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7567` |
| 17 | 1.11% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 19 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.60% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

