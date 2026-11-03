# JVM hotspots (JFR ExecutionSample)

Samples: 23536

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 2 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.05% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.31% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 5 | 2.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 7 | 1.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 14 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 18 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 27 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 31 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 37 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 40 | 0.48% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 44 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 48 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10281` |
| 50 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 54 | 0.40% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 55 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 56 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 57 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 59 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.34% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 4.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 4.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 5 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.22% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 9 | 1.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 13 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 15 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 0.94% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 24 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.65% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6979` |
| 29 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 30 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

