# JVM hotspots (JFR ExecutionSample)

Samples: 45645

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 4 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 6 | 2.34% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 7 | 1.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 11 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 31 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.50% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7797` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 42 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 45 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 47 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4901` |
| 49 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 53 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 7 | 1.88% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.88% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 12 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 18 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.86% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |

GC pauses: 0, total 0 ms

