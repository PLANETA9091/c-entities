# JVM hotspots (JFR ExecutionSample)

Samples: 62903

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 5 | 2.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 7 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 10 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 16 | 0.86% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 17 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 22 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 25 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.52% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 31 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 38 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 41 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 42 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 43 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.34% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 46 | 0.34% | `java.lang.ref.Reference.get() line: 361` |
| 47 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 48 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 51 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 52 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 53 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 54 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 56 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 1.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 7 | 1.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 9 | 1.31% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 13 | 1.11% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 15 | 0.94% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 16 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

